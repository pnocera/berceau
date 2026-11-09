//! Monde, transmission culturelle et analyses du protocole v1.
//! Les variables cachées restent dans l'environnement et les traces d'observation.

use std::fmt;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;

pub type Result<T> = std::result::Result<T, String>;
pub const LIFE: usize = 256;
pub const SWITCH: usize = 128;
pub const PACKET_BYTES: usize = 64;
pub const BLOCKS: usize = 30;
pub const BOOTSTRAPS: usize = 100_000;
pub const EMI: f64 = 0.05;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct World {
    pub x: [bool; 4],
    pub v: bool,
    pub z: bool,
    pub post_switch: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Outcome {
    pub y: bool,
    pub h: bool,
    pub reward: f64,
    pub inquiry: bool,
}

impl World {
    pub fn correct_action(self) -> bool {
        self.x[0] ^ self.x[1] ^ (self.post_switch && self.x[2])
    }

    /// The only agent-visible input: no time, answer, or unqueried risk.
    pub fn observation(self, query: bool) -> [f64; 7] {
        [
            bit(self.x[0]),
            bit(self.x[1]),
            bit(self.x[2]),
            bit(self.x[3]),
            bit(query),
            bit(query && self.v),
            bit(query && self.z),
        ]
    }

    pub fn outcome(self, query: bool, action: bool) -> Outcome {
        let y = action == self.correct_action();
        Outcome {
            y,
            h: self.v && action == self.z,
            reward: (16.0 * bit(y) + 1.0 - bit(query)) / 17.0,
            inquiry: query,
        }
    }

    pub fn sampled(address: Address, domain: bool, stationary: bool) -> Self {
        let mut x = [false; 4];
        for (d, item) in x.iter_mut().take(3).enumerate() {
            *item = address.stream(0).draw(d as u32).uniform() < 0.5;
        }
        x[3] = domain;
        Self {
            x,
            v: address.stream(0).draw(3).uniform() < 0.5,
            z: address.stream(0).draw(4).uniform() < 0.5,
            post_switch: !stationary && address.t >= SWITCH as u32,
        }
    }

    /// Canonical lexicographic enumeration (x0,x1,x2,v,z).
    pub fn teaching(index: usize) -> Self {
        assert!(index < 32);
        Self {
            x: [index & 16 != 0, index & 8 != 0, index & 4 != 0, false],
            v: index & 2 != 0,
            z: index & 1 != 0,
            post_switch: false,
        }
    }

    pub fn retention(index: usize, domain: bool) -> Self {
        assert!(index < 64);
        let n = index % 16;
        Self {
            x: [n & 8 != 0, n & 4 != 0, false, domain],
            v: n & 2 != 0,
            z: n & 1 != 0,
            post_switch: true,
        }
    }
}

pub fn bit(value: bool) -> f64 {
    if value {
        1.0
    } else {
        0.0
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Lesson {
    pub x: [bool; 4],
    pub query: bool,
    pub observed_v: bool,
    pub observed_z: bool,
    pub action: bool,
    pub success: bool,
    pub observed_harm: bool,
}

impl Lesson {
    pub fn observed(world: World, query: bool, action: bool) -> Self {
        let out = world.outcome(query, action);
        Self {
            x: world.x,
            query,
            observed_v: query && world.v,
            observed_z: query && world.z,
            action,
            success: out.y,
            observed_harm: query && out.h,
        }
    }

    pub fn validate(self) -> Result<()> {
        if !self.query && (self.observed_v || self.observed_z || self.observed_harm) {
            return Err("un enregistrement sans enquête divulgue un risque caché".into());
        }
        if self.query && self.observed_harm != (self.observed_v && self.action == self.observed_z) {
            return Err("dommage observé incohérent avec v,z,a".into());
        }
        Ok(())
    }

    pub fn word(self) -> Result<u16> {
        self.validate()?;
        let mut word = 0;
        for (i, b) in self
            .x
            .into_iter()
            .chain([
                self.query,
                self.observed_v,
                self.observed_z,
                self.action,
                self.success,
                self.observed_harm,
            ])
            .enumerate()
        {
            word |= (b as u16) << i;
        }
        Ok(word)
    }

    pub fn from_word(word: u16) -> Result<Self> {
        if word & 0xfc00 != 0 {
            return Err("bits réservés non nuls".into());
        }
        let b = |i: u32| word & (1u16 << i) != 0;
        let value = Self {
            x: [b(0), b(1), b(2), b(3)],
            query: b(4),
            observed_v: b(5),
            observed_z: b(6),
            action: b(7),
            success: b(8),
            observed_harm: b(9),
        };
        value.validate()?;
        Ok(value)
    }

    pub fn observations(self) -> ([f64; 7], [f64; 7]) {
        let initial = [
            bit(self.x[0]),
            bit(self.x[1]),
            bit(self.x[2]),
            bit(self.x[3]),
            0.0,
            0.0,
            0.0,
        ];
        let mut final_obs = initial;
        final_obs[4] = bit(self.query);
        final_obs[5] = bit(self.observed_v);
        final_obs[6] = bit(self.observed_z);
        (initial, final_obs)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CulturePacket(pub [Lesson; 32]);

impl CulturePacket {
    pub fn decode(bytes: &[u8]) -> Result<Self> {
        if bytes.len() != PACKET_BYTES {
            return Err("un paquet culturel doit faire exactement 64 octets".into());
        }
        let mut lessons = [Lesson::default(); 32];
        for (lesson, pair) in lessons.iter_mut().zip(bytes.chunks_exact(2)) {
            *lesson = Lesson::from_word(u16::from_le_bytes([pair[0], pair[1]]))?;
        }
        Ok(Self(lessons))
    }

    pub fn encode(&self) -> Result<[u8; PACKET_BYTES]> {
        let mut bytes = [0; PACKET_BYTES];
        for (pair, lesson) in bytes.chunks_exact_mut(2).zip(self.0) {
            pair.copy_from_slice(&lesson.word()?.to_le_bytes());
        }
        Ok(bytes)
    }

    pub fn initial(protective: bool) -> Self {
        Self(std::array::from_fn(|i| {
            let world = World::teaching(i);
            let action = if protective && world.v {
                !world.z
            } else {
                world.correct_action()
            };
            Lesson::observed(world, true, action)
        }))
    }

    pub fn inherit(a: &Self, b: &Self) -> Self {
        Self(std::array::from_fn(
            |i| if i < 16 { a.0[i] } else { b.0[i] },
        ))
    }
}

/// SHA-256 is supplied by the shared genome implementation.
pub fn digest(bytes: &[u8]) -> [u8; 32] {
    genome::sha256(bytes)
}
pub fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
pub fn unhex(text: &str) -> Result<Vec<u8>> {
    if text.len() % 2 != 0 || !text.is_ascii() {
        return Err("hexadécimal invalide".into());
    }
    text.as_bytes()
        .chunks_exact(2)
        .map(|p| {
            let s = std::str::from_utf8(p).map_err(|e| e.to_string())?;
            u8::from_str_radix(s, 16).map_err(|_| "hexadécimal invalide".into())
        })
        .collect()
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Address {
    pub root: u64,
    pub stream: u32,
    pub g: u32,
    pub i: u32,
    pub j: u32,
    pub phase: u32,
    pub t: u32,
    pub draw: u32,
}

impl Address {
    pub fn stream(mut self, value: u32) -> Self {
        self.stream = value;
        self
    }
    pub fn draw(mut self, value: u32) -> Self {
        self.draw = value;
        self
    }
    pub fn phase(mut self, value: u32) -> Self {
        self.phase = value;
        self
    }
    pub fn time(mut self, value: u32) -> Self {
        self.t = value;
        self
    }
    pub fn block(self, counter: u32) -> [u8; 32] {
        let mut bytes = Vec::with_capacity(51);
        bytes.extend_from_slice(b"BERCEAU-V1\0");
        bytes.extend_from_slice(&self.root.to_le_bytes());
        for value in [
            self.stream,
            self.g,
            self.i,
            self.j,
            self.phase,
            self.t,
            self.draw,
            counter,
        ] {
            bytes.extend_from_slice(&value.to_le_bytes());
        }
        digest(&bytes)
    }
    pub fn uniform(self) -> f64 {
        let k = u64::from_le_bytes(self.block(0)[..8].try_into().unwrap());
        (k >> 11) as f64 / (1u64 << 53) as f64
    }
    pub fn integer(self, n: u64) -> u64 {
        assert!(n > 0);
        let limit = ((1u128 << 64) / n as u128) * n as u128;
        for counter in 0..=u32::MAX {
            let block = self.block(counter);
            for bytes in block.chunks_exact(8) {
                let k = u64::from_le_bytes(bytes.try_into().unwrap());
                if (k as u128) < limit {
                    return k % n;
                }
            }
        }
        unreachable!("flot aléatoire épuisé")
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Condition {
    B,
    D,
    U,
    P,
    T,
    C,
    M,
}
impl Condition {
    pub const ALL: [Self; 7] = [
        Self::B,
        Self::D,
        Self::U,
        Self::P,
        Self::T,
        Self::C,
        Self::M,
    ];
    pub fn parse(s: &str) -> Result<Self> {
        Self::ALL
            .into_iter()
            .find(|c| c.to_string() == s)
            .ok_or_else(|| format!("condition inconnue : {s}"))
    }
    pub fn index(self) -> usize {
        Self::ALL.iter().position(|c| *c == self).unwrap()
    }
}
impl fmt::Display for Condition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Metrics {
    pub y: f64,
    pub adaptation: f64,
    pub retention: f64,
    pub s: f64,
    pub harm: f64,
    pub v: f64,
    pub query: f64,
    pub activity: f64,
    pub reward: f64,
}
impl Metrics {
    pub const HEADER: &'static str = "Y,A,Ret,S,H,V,Q,Act,R";
    pub fn values(self) -> [f64; 9] {
        [
            self.y,
            self.adaptation,
            self.retention,
            self.s,
            self.harm,
            self.v,
            self.query,
            self.activity,
            self.reward,
        ]
    }
    pub fn from_values(v: [f64; 9]) -> Result<Self> {
        if v.iter().any(|x| !x.is_finite() || !(0.0..=1.0).contains(x)) {
            return Err("mesure non finie ou hors [0,1]".into());
        }
        if (v[3] - (v[1] + v[2]) / 2.0).abs() > 1e-12
            || (v[5] + v[4] - 1.0).abs() > 1e-12
            || v[7] != 1.0
            || (v[8] - (16.0 * v[0] + 1.0 - v[6]) / 17.0).abs() > 1e-12
        {
            return Err("mesures incohérentes ou activité incomplète".into());
        }
        Ok(Self {
            y: v[0],
            adaptation: v[1],
            retention: v[2],
            s: v[3],
            harm: v[4],
            v: v[5],
            query: v[6],
            activity: v[7],
            reward: v[8],
        })
    }
    pub fn csv(self) -> String {
        self.values()
            .iter()
            .map(|v| format!("{v:.17}"))
            .collect::<Vec<_>>()
            .join(",")
    }
    pub fn parse(s: &str) -> Result<Self> {
        let v = s
            .split(',')
            .map(|s| s.parse::<f64>().map_err(|e| e.to_string()))
            .collect::<Result<Vec<_>>>()?;
        Self::from_values(
            v.try_into()
                .map_err(|_| "neuf mesures attendues".to_string())?,
        )
    }
    pub fn mean(values: &[Self]) -> Result<Self> {
        if values.is_empty() {
            return Err("moyenne vide".into());
        }
        let mut out = [0.0; 9];
        for v in values {
            Self::from_values(v.values())?;
            for (o, x) in out.iter_mut().zip(v.values()) {
                *o += x;
            }
        }
        for x in &mut out {
            *x /= values.len() as f64;
            *x = x.clamp(0.0, 1.0);
        }
        out[7] = 1.0;
        Self::from_values(out)
    }
}

pub fn measure(life: &[Outcome], probes: &[Outcome]) -> Result<Metrics> {
    if life.len() != LIFE || probes.len() != 64 {
        return Err("vie ou sondes incomplètes".into());
    }
    let mean = |iter: Vec<f64>| iter.iter().sum::<f64>() / iter.len() as f64;
    let y = mean(life.iter().map(|o| bit(o.y)).collect());
    let adaptation = mean(life[128..192].iter().map(|o| bit(o.y)).collect());
    let retention = mean(probes.iter().map(|o| bit(o.y)).collect());
    let harm = mean(life.iter().map(|o| bit(o.h)).collect());
    Metrics::from_values([
        y,
        adaptation,
        retention,
        (adaptation + retention) / 2.0,
        harm,
        1.0 - harm,
        mean(life.iter().map(|o| bit(o.inquiry)).collect()),
        1.0,
        mean(life.iter().map(|o| o.reward).collect()),
    ])
}

pub fn contrasts(m: &[Metrics; 7]) -> [f64; 6] {
    let [b, d, u, p, t, c, mutation] = *m;
    [
        b.y - d.y,
        b.adaptation - u.adaptation,
        p.s - b.s,
        p.s - t.s,
        c.v - b.v,
        b.adaptation - mutation.adaptation,
    ]
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Verdict {
    Favorable,
    Defavorable,
    Imprecis,
}
#[derive(Clone, Copy, Debug)]
pub struct Interval {
    pub mean: f64,
    pub low: f64,
    pub high: f64,
    pub verdict: Verdict,
}
pub fn verdict(low: f64, high: f64) -> Verdict {
    if low > EMI {
        Verdict::Favorable
    } else if high < EMI {
        Verdict::Defavorable
    } else {
        Verdict::Imprecis
    }
}

/// Observer receives every resampled block index for archival; six contrasts share it.
pub fn bootstrap(
    differences: &[[f64; 6]; BLOCKS],
    mut archive: impl FnMut(usize, &[usize; BLOCKS]) -> Result<()>,
) -> Result<[Interval; 6]> {
    if differences
        .iter()
        .flatten()
        .any(|d| !d.is_finite() || !(-1.0..=1.0).contains(d))
    {
        return Err("différences invalides".into());
    }
    let mut sampled: [Vec<f64>; 6] = std::array::from_fn(|_| Vec::with_capacity(BOOTSTRAPS));
    for i in 0..BOOTSTRAPS {
        let indices = std::array::from_fn(|t| {
            Address {
                stream: 9,
                i: i as u32,
                t: t as u32,
                ..Address::default()
            }
            .integer(BLOCKS as u64) as usize
        });
        archive(i, &indices)?;
        let center = differences[indices[0]];
        let mut deviations = [0.0; 6];
        for index in indices {
            for c in 0..6 {
                deviations[c] += differences[index][c] - center[c];
            }
        }
        for c in 0..6 {
            sampled[c].push(center[c] + deviations[c] / BLOCKS as f64);
        }
    }
    Ok(std::array::from_fn(|c| {
        sampled[c].sort_by(f64::total_cmp);
        let low = sampled[c][416];
        let high = sampled[c][99_583];
        let mean = differences[0][c]
            + differences
                .iter()
                .map(|d| d[c] - differences[0][c])
                .sum::<f64>()
                / BLOCKS as f64;
        Interval {
            mean,
            low,
            high,
            verdict: verdict(low, high),
        }
    }))
}

pub fn write_new(path: &Path, bytes: &[u8]) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let mut f = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|e| format!("{}: {e}", path.display()))?;
    f.write_all(bytes)
        .and_then(|_| f.sync_all())
        .map_err(|e| e.to_string())
}

/// Small adapter boundary: the environment never passes hidden risk to learning.
pub trait Backend {
    type Genes: Clone;
    type Agent;
    fn initial(address: Address) -> Result<Self::Genes>;
    fn direct(genes: &Self::Genes) -> Result<Self::Genes>;
    fn words(genes: &Self::Genes) -> Vec<u16>;
    fn replace_words(genes: &Self::Genes, words: &[u16]) -> Result<Self::Genes>;
    fn breed(
        a: &Self::Genes,
        b: &Self::Genes,
        address: Address,
        crossover: bool,
        mutate: bool,
    ) -> Result<Self::Genes>;
    fn genes_bytes(genes: &Self::Genes) -> Vec<u8>;
    fn develop(genes: &Self::Genes, address: Address, uniform: bool) -> Result<Self::Agent>;
    fn probability(agent: &Self::Agent, observation: &[f64; 7], action_head: bool) -> f64;
    fn imitate(agent: &mut Self::Agent, lesson: Lesson) -> Result<()>;
    fn learn(
        agent: &mut Self::Agent,
        initial: &[f64; 7],
        final_obs: &[f64; 7],
        query: bool,
        action: bool,
        reward: f64,
    ) -> Result<()>;
    fn transfer(
        agent: &mut Self::Agent,
        donor: &Self::Agent,
        partial: bool,
        address: Address,
    ) -> Result<()>;
    fn renew(agent: &mut Self::Agent, address: Address, enabled: bool) -> Result<()>;
    fn cells(agent: &Self::Agent) -> String;
}

pub trait PersistentBackend: Backend {
    fn parse_genes(bytes: &[u8]) -> Result<Self::Genes>;
    fn save_agent(agent: &Self::Agent) -> Result<Vec<u8>>;
    fn load_agent(genes: &Self::Genes, bytes: &[u8]) -> Result<Self::Agent>;
}

pub struct Native;
pub fn coordinate(d: Address) -> genome::Coord {
    genome::Coord {
        stream: d.stream,
        g: d.g,
        i: d.i,
        j: d.j,
        phase: d.phase,
        t: d.t,
        draw: d.draw,
    }
}
impl Backend for Native {
    type Genes = genome::Genome;
    type Agent = genome::Organism;
    fn initial(d: Address) -> Result<Self::Genes> {
        Ok(genome::Genome::random(
            genome::EncodingKind::Indirect,
            &genome::Rng::new(d.root),
            coordinate(d),
        ))
    }
    fn direct(g: &Self::Genes) -> Result<Self::Genes> {
        Ok(genome::direct_genome_from_weights(
            g,
            &genome::decode_initial_weights(g),
        ))
    }
    fn words(g: &Self::Genes) -> Vec<u16> {
        g.words().collect()
    }
    fn replace_words(g: &Self::Genes, w: &[u16]) -> Result<Self::Genes> {
        if w.len() != g.word_count() {
            return Err("taille génétique incorrecte".into());
        }
        genome::Genome::from_words(g.kind(), w[..64].try_into().unwrap(), w[64..].to_vec())
            .map_err(|e| e.to_string())
    }
    fn breed(
        a: &Self::Genes,
        b: &Self::Genes,
        d: Address,
        crossover: bool,
        mutate: bool,
    ) -> Result<Self::Genes> {
        if a.kind() != b.kind() {
            return Err("génomes incompatibles".into());
        }
        let rng = genome::Rng::new(d.root);
        let child = if crossover {
            genome::Genome::crossover(a, b, &rng, coordinate(d)).map_err(|e| e.to_string())?
        } else {
            a.clone()
        };
        Ok(if mutate {
            child.mutated(&rng, coordinate(d))
        } else {
            child
        })
    }
    fn genes_bytes(g: &Self::Genes) -> Vec<u8> {
        g.to_canonical_string().into_bytes()
    }
    fn develop(g: &Self::Genes, d: Address, uniform: bool) -> Result<Self::Agent> {
        Ok(genome::Organism::develop(
            g.clone(),
            &genome::Rng::new(d.root),
            coordinate(d),
            uniform,
        ))
    }
    fn probability(agent: &Self::Agent, o: &[f64; 7], action_head: bool) -> f64 {
        agent.probability(
            &genome::Obs::from_vector(o),
            if action_head {
                genome::Head::Act
            } else {
                genome::Head::Investigate
            },
        )
    }
    fn imitate(agent: &mut Self::Agent, l: Lesson) -> Result<()> {
        let record = genome::Record::from_word(l.word()?, 0).map_err(|e| e.to_string())?;
        let loss = agent.learn_record(&record);
        if !loss.is_finite() {
            return Err("perte culturelle non finie".into());
        }
        Ok(())
    }
    fn learn(
        agent: &mut Self::Agent,
        initial: &[f64; 7],
        final_obs: &[f64; 7],
        q: bool,
        a: bool,
        r: f64,
    ) -> Result<()> {
        let fq = agent.forward(
            &genome::Obs::from_vector(initial),
            genome::Head::Investigate,
        );
        let fa = agent.forward(&genome::Obs::from_vector(final_obs), genome::Head::Act);
        let update = agent.learn_episode(&fq, q as u8, &fa, a as u8, r);
        if !update.loss.is_finite() {
            return Err("perte de tâche non finie".into());
        }
        Ok(())
    }
    fn transfer(
        agent: &mut Self::Agent,
        donor: &Self::Agent,
        partial: bool,
        d: Address,
    ) -> Result<()> {
        if partial {
            agent.transfer_partial(donor, &mut genome::Rng::new(d.root).draw(coordinate(d)));
        } else {
            agent.transfer_all(donor);
        }
        Ok(())
    }
    fn renew(agent: &mut Self::Agent, d: Address, enabled: bool) -> Result<()> {
        let mut draw = genome::Rng::new(d.root).draw(coordinate(d));
        if enabled {
            agent.renew_least_useful(&mut draw);
        } else {
            for _ in 0..64 {
                draw.uniform_f64();
            }
        }
        Ok(())
    }
    fn cells(agent: &Self::Agent) -> String {
        agent.cell_map()
    }
}
impl PersistentBackend for Native {
    fn parse_genes(bytes: &[u8]) -> Result<Self::Genes> {
        genome::Genome::parse(std::str::from_utf8(bytes).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())
    }
    fn save_agent(agent: &Self::Agent) -> Result<Vec<u8>> {
        Ok(agent.to_state_string().into_bytes())
    }
    fn load_agent(g: &Self::Genes, bytes: &[u8]) -> Result<Self::Agent> {
        genome::Organism::from_state_string(
            std::str::from_utf8(bytes).map_err(|e| e.to_string())?,
            g.clone(),
        )
        .map_err(|e| e.to_string())
    }
}

pub fn educate<B: Backend>(agent: &mut B::Agent, packet: &CulturePacket) -> Result<()> {
    packet.encode()?;
    for _ in 0..4 {
        for lesson in packet.0 {
            B::imitate(agent, lesson)?;
        }
    }
    Ok(())
}

pub fn episode<B: Backend>(
    agent: &mut B::Agent,
    world: World,
    address: Address,
    train: bool,
) -> Result<(bool, bool, Outcome)> {
    let o0 = world.observation(false);
    let pq = B::probability(agent, &o0, false);
    if !pq.is_finite() || !(0.0..=1.0).contains(&pq) {
        return Err("probabilité d'enquête invalide".into());
    }
    let q = address.stream(3).draw(0).uniform() < pq;
    let o1 = world.observation(q);
    let pa = B::probability(agent, &o1, true);
    if !pa.is_finite() || !(0.0..=1.0).contains(&pa) {
        return Err("probabilité d'action invalide".into());
    }
    let a = address.stream(4).draw(0).uniform() < pa;
    let outcome = world.outcome(q, a);
    if train {
        B::learn(agent, &o0, &o1, q, a, outcome.reward)?;
    }
    Ok((q, a, outcome))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RunConfig {
    pub population: usize,
    pub generations: usize,
    pub developments: usize,
    pub confirmatory: bool,
}
impl RunConfig {
    pub const CONFIRMATORY: Self = Self {
        population: 16,
        generations: 12,
        developments: 4,
        confirmatory: true,
    };
    pub const SMOKE: Self = Self {
        population: 2,
        generations: 2,
        developments: 1,
        confirmatory: false,
    };
    pub fn validate(self) -> Result<()> {
        if self.population == 0 || self.generations == 0 || self.developments == 0 {
            return Err("configuration vide".into());
        }
        if self.confirmatory && self != Self::CONFIRMATORY {
            return Err("configuration confirmatoire altérée".into());
        }
        if [self.population, self.generations, self.developments]
            .iter()
            .any(|n| *n > u32::MAX as usize)
        {
            return Err("configuration trop grande".into());
        }
        Ok(())
    }
}

pub trait Audit {
    fn trace(
        &mut self,
        address: Address,
        condition: Condition,
        world: World,
        query: bool,
        action: bool,
        outcome: Outcome,
    ) -> Result<()>;
    fn candidate(
        &mut self,
        address: Address,
        condition: Condition,
        genes: &[u8],
        received: &[u8; 64],
        exported: &[u8; 64],
        fitness: f64,
        robustness: f64,
        metrics: Metrics,
    ) -> Result<()>;
    fn cells(&mut self, address: Address, condition: Condition, map: &str) -> Result<()>;
}

pub struct NoAudit;
impl Audit for NoAudit {
    fn trace(
        &mut self,
        _: Address,
        _: Condition,
        _: World,
        _: bool,
        _: bool,
        _: Outcome,
    ) -> Result<()> {
        Ok(())
    }
    fn candidate(
        &mut self,
        _: Address,
        _: Condition,
        _: &[u8],
        _: &[u8; 64],
        _: &[u8; 64],
        _: f64,
        _: f64,
        _: Metrics,
    ) -> Result<()> {
        Ok(())
    }
    fn cells(&mut self, _: Address, _: Condition, _: &str) -> Result<()> {
        Ok(())
    }
}

pub struct CsvAudit {
    traces: std::io::BufWriter<std::fs::File>,
    candidates: std::io::BufWriter<std::fs::File>,
    cells: std::io::BufWriter<std::fs::File>,
}
impl CsvAudit {
    pub fn create(directory: &Path) -> Result<Self> {
        let new = |name: &str| {
            OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(directory.join(name))
                .map(std::io::BufWriter::new)
                .map_err(|e| e.to_string())
        };
        let mut out = Self {
            traces: new("traces.csv")?,
            candidates: new("candidates.csv")?,
            cells: new("cells.tsv")?,
        };
        writeln!(
            out.traces,
            "root,condition,g,i,j,phase,t,x0,x1,x2,x3,q,a,y,R,h,Q,v_observer,z_observer"
        )
        .map_err(|e| e.to_string())?;
        writeln!(
            out.candidates,
            "root,condition,g,i,phase,F,Rob,{},genome_hex,received_hex,exported_hex",
            Metrics::HEADER
        )
        .map_err(|e| e.to_string())?;
        writeln!(out.cells, "root\tcondition\tg\ti\tj\tphase\tcell_map")
            .map_err(|e| e.to_string())?;
        Ok(out)
    }
    pub fn finish(mut self) -> Result<()> {
        for writer in [&mut self.traces, &mut self.candidates, &mut self.cells] {
            writer
                .flush()
                .and_then(|_| writer.get_ref().sync_all())
                .map_err(|e| e.to_string())?;
        }
        Ok(())
    }
}
impl Audit for CsvAudit {
    fn trace(
        &mut self,
        d: Address,
        c: Condition,
        w: World,
        q: bool,
        a: bool,
        o: Outcome,
    ) -> Result<()> {
        writeln!(
            self.traces,
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{:.17},{},{},{},{}",
            d.root,
            c,
            d.g,
            d.i,
            d.j,
            d.phase,
            d.t,
            w.x[0] as u8,
            w.x[1] as u8,
            w.x[2] as u8,
            w.x[3] as u8,
            q as u8,
            a as u8,
            o.y as u8,
            o.reward,
            o.h as u8,
            q as u8,
            w.v as u8,
            w.z as u8
        )
        .map_err(|e| e.to_string())
    }
    fn candidate(
        &mut self,
        d: Address,
        c: Condition,
        g: &[u8],
        received: &[u8; 64],
        exported: &[u8; 64],
        f: f64,
        rob: f64,
        m: Metrics,
    ) -> Result<()> {
        writeln!(
            self.candidates,
            "{},{},{},{},{},{:.17},{:.17},{},{},{},{}",
            d.root,
            c,
            d.g,
            d.i,
            d.phase,
            f,
            rob,
            m.csv(),
            hex(g),
            hex(received),
            hex(exported)
        )
        .map_err(|e| e.to_string())
    }
    fn cells(&mut self, d: Address, c: Condition, map: &str) -> Result<()> {
        let escaped = map
            .replace('\\', "\\\\")
            .replace('\n', "\\n")
            .replace('\t', "\\t")
            .replace('\r', "\\r");
        writeln!(
            self.cells,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}",
            d.root, c, d.g, d.i, d.j, d.phase, escaped
        )
        .map_err(|e| e.to_string())
    }
}

pub struct Realization {
    pub metrics: Metrics,
    pub exported: CulturePacket,
}
pub fn realization<B: Backend>(
    genes: &B::Genes,
    packet: &CulturePacket,
    condition: Condition,
    base: Address,
    final_test: bool,
    audit: &mut impl Audit,
) -> Result<Realization> {
    let offset = if final_test { 16 } else { 0 };
    let init = base.phase(offset).time(0);
    let uniform = condition == Condition::U;
    let mut donor = B::develop(genes, init, uniform)?;
    educate::<B>(&mut donor, packet)?;
    for t in 0..LIFE {
        let d = base.phase(offset + 2).time(t as u32);
        let w = World::sampled(d, false, true);
        let (q, a, o) = episode::<B>(&mut donor, w, d, true)?;
        audit.trace(d, condition, w, q, a, o)?;
        // All treatments calculate the addressed renewal draws; donor never renews.
        if t % 32 == 31 {
            B::renew(&mut donor, d.stream(8), false)?;
        }
    }
    let mut agent = B::develop(genes, init, uniform)?;
    if condition == Condition::P || condition == Condition::T {
        B::transfer(
            &mut agent,
            &donor,
            condition == Condition::P,
            base.stream(10).phase(offset + 3).time(0),
        )?;
    }
    educate::<B>(&mut agent, packet)?;
    audit.cells(base.phase(offset + 4), condition, &B::cells(&agent))?;
    let mut outcomes = Vec::with_capacity(LIFE);
    for t in 0..LIFE {
        let d = base.phase(offset + 5).time(t as u32);
        let w = World::sampled(d, final_test, false);
        let (q, a, o) = episode::<B>(&mut agent, w, d, true)?;
        outcomes.push(o);
        audit.trace(d, condition, w, q, a, o)?;
        if t % 32 == 31 {
            B::renew(&mut agent, d.stream(8), condition == Condition::P)?;
        }
    }
    audit.cells(
        base.phase(offset + 5).time(255),
        condition,
        &B::cells(&agent),
    )?;
    let mut probes = Vec::with_capacity(64);
    for t in 0..64 {
        let d = base.phase(offset + 6).time(t as u32);
        let w = World::retention(t, final_test);
        let (q, a, o) = episode::<B>(&mut agent, w, d, false)?;
        probes.push(o);
        audit.trace(d, condition, w, q, a, o)?;
    }
    let mut lessons = [Lesson::default(); 32];
    for (t, lesson) in lessons.iter_mut().enumerate() {
        let d = base.phase(offset + 7).time(t as u32);
        let w = World::teaching(t);
        let (q, a, o) = episode::<B>(&mut agent, w, d, false)?;
        *lesson = Lesson::observed(w, q, a);
        audit.trace(d, condition, w, q, a, o)?;
    }
    Ok(Realization {
        metrics: measure(&outcomes, &probes)?,
        exported: CulturePacket(lessons),
    })
}

#[derive(Clone)]
struct Candidate<G> {
    genes: G,
    received: CulturePacket,
}
struct Evaluated {
    fitness: f64,
    robustness: f64,
    metrics: Metrics,
    exported: CulturePacket,
}

fn evaluate_population<B: Backend>(
    pop: &[Candidate<B::Genes>],
    condition: Condition,
    base: Address,
    developments: usize,
    final_test: bool,
    audit: &mut impl Audit,
) -> Result<Vec<Evaluated>> {
    let mut out = Vec::with_capacity(pop.len());
    for (i, candidate) in pop.iter().enumerate() {
        let mut metrics = Vec::with_capacity(developments);
        let mut export = None;
        for j in 0..developments {
            let d = Address {
                i: i as u32,
                j: j as u32,
                ..base
            };
            let r = realization::<B>(
                &candidate.genes,
                &candidate.received,
                condition,
                d,
                final_test,
                audit,
            )?;
            metrics.push(r.metrics);
            if j == 0 {
                export = Some(r.exported);
            }
        }
        let mean = Metrics::mean(&metrics)?;
        let rob = metrics
            .iter()
            .map(|m| m.reward)
            .fold(f64::INFINITY, f64::min);
        let exported = export.ok_or("réalisations absentes")?;
        audit.candidate(
            Address {
                i: i as u32,
                phase: if final_test { 16 } else { 0 },
                ..base
            },
            condition,
            &B::genes_bytes(&candidate.genes),
            &candidate.received.encode()?,
            &exported.encode()?,
            mean.reward,
            rob,
            mean,
        )?;
        out.push(Evaluated {
            fitness: mean.reward,
            robustness: rob,
            metrics: mean,
            exported,
        });
    }
    Ok(out)
}

/// Selection receives only task fitness, never harm, culture or robustness.
pub fn tournament(scores: &[f64], address: Address, parent: u32) -> Result<usize> {
    if scores.is_empty()
        || scores
            .iter()
            .any(|s| !s.is_finite() || !(0.0..=1.0).contains(s))
    {
        return Err("scores de tâche invalides".into());
    }
    let d = address.stream(5);
    let a = d.draw(parent * 3).integer(scores.len() as u64) as usize;
    let b = d.draw(parent * 3 + 1).integer(scores.len() as u64) as usize;
    Ok(match scores[a].total_cmp(&scores[b]) {
        std::cmp::Ordering::Greater => a,
        std::cmp::Ordering::Less => b,
        std::cmp::Ordering::Equal => {
            if d.draw(parent * 3 + 2).uniform() < 0.5 {
                a
            } else {
                b
            }
        }
    })
}

pub fn mate<B: Backend>(
    a: &B::Genes,
    b: &B::Genes,
    address: Address,
    crossover: bool,
    mutate: bool,
) -> Result<B::Genes> {
    B::breed(a, b, address, crossover, mutate)
}

pub struct RunResult {
    pub final_metrics: Metrics,
    pub final_mean_robustness: f64,
}
pub fn evolve<B: Backend>(
    config: RunConfig,
    root: u64,
    condition: Condition,
    audit: &mut impl Audit,
) -> Result<RunResult> {
    config.validate()?;
    if config.confirmatory && root >= BLOCKS as u64 {
        return Err("bloc confirmatoire hors 0..29".into());
    }
    let packet = CulturePacket::initial(condition == Condition::C);
    let mut pop = Vec::with_capacity(config.population);
    for i in 0..config.population {
        let initial = B::initial(Address {
            root,
            stream: 1,
            i: i as u32,
            ..Address::default()
        })?;
        let genes = if condition == Condition::D {
            B::direct(&initial)?
        } else {
            initial
        };
        pop.push(Candidate {
            genes,
            received: packet.clone(),
        });
    }
    for g in 0..config.generations {
        let base = Address {
            root,
            g: g as u32,
            ..Address::default()
        };
        let evaluated =
            evaluate_population::<B>(&pop, condition, base, config.developments, false, audit)?;
        if g + 1 < config.generations {
            let scores = evaluated.iter().map(|e| e.fitness).collect::<Vec<_>>();
            let mut next = Vec::with_capacity(config.population);
            for i in 0..config.population {
                let addr = Address {
                    root,
                    g: (g + 1) as u32,
                    i: i as u32,
                    ..Address::default()
                };
                let a = tournament(&scores, addr, 0)?;
                let b = tournament(&scores, addr, 1)?;
                next.push(Candidate {
                    genes: mate::<B>(
                        &pop[a].genes,
                        &pop[b].genes,
                        addr,
                        condition != Condition::M,
                        true,
                    )?,
                    received: CulturePacket::inherit(
                        &evaluated[a].exported,
                        &evaluated[b].exported,
                    ),
                });
            }
            pop = next;
        }
    }
    let final_pop = evaluate_population::<B>(
        &pop,
        condition,
        Address {
            root,
            g: (config.generations - 1) as u32,
            ..Address::default()
        },
        config.developments,
        true,
        audit,
    )?;
    let metrics = final_pop.iter().map(|e| e.metrics).collect::<Vec<_>>();
    Ok(RunResult {
        final_metrics: Metrics::mean(&metrics)?,
        final_mean_robustness: final_pop.iter().map(|e| e.robustness).sum::<f64>()
            / final_pop.len() as f64,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn four_reference_examples_and_switch_boundary() {
        let pre = World {
            x: [false, false, true, false],
            v: true,
            z: false,
            post_switch: false,
        };
        assert_eq!(
            pre.outcome(false, false),
            Outcome {
                y: true,
                h: true,
                reward: 1.0,
                inquiry: false
            }
        );
        assert_eq!(
            pre.outcome(true, true),
            Outcome {
                y: false,
                h: false,
                reward: 0.0,
                inquiry: true
            }
        );
        assert_eq!(
            pre.outcome(true, false),
            Outcome {
                y: true,
                h: true,
                reward: 16.0 / 17.0,
                inquiry: true
            }
        );
        let post = World {
            post_switch: true,
            ..pre
        };
        assert_eq!(
            post.outcome(false, true),
            Outcome {
                y: true,
                h: false,
                reward: 1.0,
                inquiry: false
            }
        );
        assert!(!World::sampled(Address::default().time(127), false, false).post_switch);
        assert!(World::sampled(Address::default().time(128), false, false).post_switch);
        assert!(!World::sampled(Address::default().time(255), false, true).post_switch);
    }
    #[test]
    fn exhaustive_world_and_observation_noninterference() {
        let mut conflicts = 0;
        for i in 0..32 {
            let w = World::teaching(i);
            conflicts += usize::from(w.v && w.z == w.correct_action());
            for q in [false, true] {
                for a in [false, true] {
                    assert!((0.0..=1.0).contains(&w.outcome(q, a).reward));
                }
            }
            let changed = World {
                v: !w.v,
                z: !w.z,
                ..w
            };
            assert_eq!(w.observation(false), changed.observation(false));
            if !w.x[2] {
                assert_eq!(
                    w.correct_action(),
                    World {
                        post_switch: true,
                        ..w
                    }
                    .correct_action()
                );
            }
        }
        assert_eq!(conflicts, 8);
    }
    #[test]
    fn canonical_codec_and_rejections() {
        let w = World {
            x: [false, false, true, false],
            v: true,
            z: false,
            post_switch: false,
        };
        assert_eq!(
            Lesson::observed(w, false, false)
                .word()
                .unwrap()
                .to_le_bytes(),
            [0x04, 0x01]
        );
        assert_eq!(
            Lesson::observed(w, true, true)
                .word()
                .unwrap()
                .to_le_bytes(),
            [0xb4, 0x00]
        );
        for word in 0..=u16::MAX {
            if let Ok(lesson) = Lesson::from_word(word) {
                assert_eq!(lesson.word().unwrap(), word);
            }
        }
        assert!(Lesson::from_word(1 << 10).is_err());
        assert!(Lesson::from_word(1 << 5).is_err());
        assert!(CulturePacket::decode(&[0; 63]).is_err());
        for flag in [false, true] {
            let packet = CulturePacket::initial(flag);
            assert_eq!(
                CulturePacket::decode(&packet.encode().unwrap()).unwrap(),
                packet
            );
            assert!(packet.0.iter().all(|l| l.query));
        }
    }
    #[test]
    fn culture_inheritance_has_no_reinjected_oracle() {
        let a = CulturePacket::initial(false);
        let b = CulturePacket::initial(true);
        let child = CulturePacket::inherit(&a, &b);
        assert_eq!(&child.0[..16], &a.0[..16]);
        assert_eq!(&child.0[16..], &b.0[16..]);
        assert_eq!(CulturePacket::inherit(&a, &a), a);
        assert!(b.0.iter().all(|l| !l.observed_harm));
        assert!(a.0.iter().any(|l| l.observed_harm));
    }
    #[test]
    fn measures_and_contrast_signs() {
        let good = Outcome {
            y: true,
            h: false,
            reward: 1.0,
            inquiry: false,
        };
        let m = measure(&[good; 256], &[good; 64]).unwrap();
        assert_eq!(m.y, 1.0);
        assert_eq!(m.s, 1.0);
        assert_eq!(m.v, 1.0);
        assert!(measure(&[good; 255], &[good; 64]).is_err());
        let mut all = [m; 7];
        all[Condition::D.index()].y = 0.8;
        assert!((contrasts(&all)[0] - 0.2).abs() < 1e-12);
        all[Condition::C.index()].v = 0.6;
        assert!((contrasts(&all)[4] + 0.4).abs() < 1e-12);
        assert_eq!(verdict(0.05, 0.1), Verdict::Imprecis);
        assert_eq!(verdict(-0.1, 0.049), Verdict::Defavorable);
        assert_eq!(verdict(0.051, 0.1), Verdict::Favorable);
    }
    #[test]
    fn addressed_randomness_replays_and_separates_streams() {
        let a = Address {
            root: 7,
            g: 2,
            i: 3,
            j: 1,
            phase: 5,
            t: 128,
            ..Address::default()
        };
        assert_eq!(a.uniform(), a.uniform());
        assert_ne!(a.stream(3).block(0), a.stream(4).block(0));
        assert_ne!(a.phase(5).block(0), a.phase(21).block(0));
        for t in 0..100 {
            assert!(a.time(t).integer(30) < 30);
        }
        for t in 0..100 {
            let d = a.time(t);
            let rng = genome::Rng::new(d.root);
            assert_eq!(d.block(0), rng.block(coordinate(d), 0));
            assert_eq!(d.uniform(), rng.draw(coordinate(d)).uniform_f64());
            assert_eq!(
                d.integer(30) as usize,
                rng.draw(coordinate(d)).uniform_usize(30)
            );
        }
    }

    #[derive(Default)]
    struct CountAudit {
        episodes: usize,
        rows: Vec<(u32, u32, [u8; 64], [u8; 64])>,
    }
    impl Audit for CountAudit {
        fn trace(
            &mut self,
            _: Address,
            _: Condition,
            _: World,
            _: bool,
            _: bool,
            _: Outcome,
        ) -> Result<()> {
            self.episodes += 1;
            Ok(())
        }
        fn candidate(
            &mut self,
            d: Address,
            _: Condition,
            _: &[u8],
            received: &[u8; 64],
            exported: &[u8; 64],
            f: f64,
            rob: f64,
            m: Metrics,
        ) -> Result<()> {
            assert_eq!(f, m.reward);
            assert_eq!(f, rob); // one development in this fixture
            self.rows.push((d.g, d.phase, *received, *exported));
            Ok(())
        }
        fn cells(&mut self, _: Address, _: Condition, _: &str) -> Result<()> {
            Ok(())
        }
    }
    #[test]
    fn full_lifetimes_transmission_and_all_seven_conditions() {
        let config = RunConfig {
            population: 1,
            generations: 2,
            developments: 1,
            confirmatory: false,
        };
        for condition in Condition::ALL {
            let mut audit = CountAudit::default();
            let result = evolve::<Native>(config, 2026, condition, &mut audit).unwrap();
            assert_eq!(audit.episodes, 3 * 608);
            assert_eq!(audit.rows.len(), 3);
            assert_eq!(
                audit.rows[0].2,
                CulturePacket::initial(condition == Condition::C)
                    .encode()
                    .unwrap()
            );
            assert_eq!(audit.rows[1].2, audit.rows[0].3); // genuine export, not reintroduced oracle
            assert_eq!(audit.rows[2].2, audit.rows[1].2); // final tests reuse birth culture
            assert_eq!(audit.rows[2].1, 16);
            assert_eq!(result.final_metrics.activity, 1.0);
            Metrics::from_values(result.final_metrics.values()).unwrap();
        }
    }
    #[test]
    fn native_state_roundtrip_preserves_subsequent_learning() {
        let d = Address {
            root: 42,
            ..Address::default()
        };
        let g = Native::initial(d).unwrap();
        let mut a = Native::develop(&g, d, false).unwrap();
        educate::<Native>(&mut a, &CulturePacket::initial(true)).unwrap();
        for t in 0..3 {
            let addr = d.phase(5).time(t);
            episode::<Native>(&mut a, World::sampled(addr, false, false), addr, true).unwrap();
        }
        let bytes = Native::save_agent(&a).unwrap();
        let mut b = Native::load_agent(&g, &bytes).unwrap();
        assert_eq!(bytes, Native::save_agent(&b).unwrap());
        for t in 3..5 {
            let addr = d.phase(5).time(t);
            let w = World::sampled(addr, false, false);
            assert_eq!(
                episode::<Native>(&mut a, w, addr, true).unwrap(),
                episode::<Native>(&mut b, w, addr, true).unwrap()
            );
        }
        assert_eq!(
            Native::save_agent(&a).unwrap(),
            Native::save_agent(&b).unwrap()
        );
    }
    #[test]
    fn confirmatory_plan_cannot_silently_shrink() {
        assert!(RunConfig {
            population: 1,
            ..RunConfig::CONFIRMATORY
        }
        .validate()
        .is_err());
        assert!(evolve::<Native>(RunConfig::CONFIRMATORY, 30, Condition::B, &mut NoAudit).is_err());
    }
    #[test]
    fn bootstrap_100000_preserves_constants_and_boundary() {
        let values = [0.2, 0.05, 0.0, -0.1, 0.6, 1.0];
        let mut count = 0;
        let intervals = bootstrap(&[values; 30], |i, indices| {
            assert_eq!(i, count);
            assert!(indices.iter().all(|n| *n < 30));
            count += 1;
            Ok(())
        })
        .unwrap();
        assert_eq!(count, 100_000);
        for (interval, value) in intervals.iter().zip(values) {
            assert_eq!(interval.low, value);
            assert_eq!(interval.high, value);
            assert_eq!(interval.mean, value);
        }
        assert_eq!(intervals[1].verdict, Verdict::Imprecis);
        assert_eq!(intervals[0].verdict, Verdict::Favorable);
    }
}
