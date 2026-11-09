//! Organisme développé : 16 cellules en anneau, 64 poids par cellule, deux têtes de
//! décision, apprentissage par renforcement et par imitation culturelle
//! (PROTOCOLE.md §3.2, §3.4, §4.2 ; SPEC_GENOME.md §5–§7).

use crate::decoder::{decode_initial_weights, INITIAL_WEIGHT_DIVISOR};
#[cfg(test)]
use crate::decoder::INITIAL_WEIGHT_BOUND;
use crate::develop::{develop, gene, RegParams, Regulation, GENES};
use crate::genome::Genome;
use crate::packet::{Packet, Record};
use crate::rng::{Coord, Draw, Rng};
use crate::{CELLS, HIDDEN, INPUT_DIM, WEIGHTS_PER_CELL};

/// Borne des poids appris.
pub const WEIGHT_CLAMP: f64 = 8.0;
/// Borne des probabilités pour les logarithmes : `[2^-16, 1 - 2^-16]`.
pub const PROB_EPS: f64 = 1.0 / 65536.0;
/// Taux d'apprentissage culturel, fixe et commun à toutes les cellules.
pub const CULTURE_RATE: f64 = 1.0 / 64.0;
/// Passages culturels sur un paquet.
pub const CULTURE_PASSES: usize = 4;
/// Lissage de la référence : `m ← (15 m + R) / 16`.
pub const BASELINE_KEEP: f64 = 15.0 / 16.0;
/// Lissage de l'utilité : `u ← 0,99 u + 0,01 · mean(|act · ∂L/∂act|)`.
pub const UTILITY_KEEP: f64 = 0.99;
/// Nombre de cellules copiées par un transfert partiel.
pub const PARTIAL_TRANSFER_CELLS: usize = 8;

/// Disposition des 64 poids d'une cellule.
pub mod layout {
    /// `W_in[r][k] = w[8 r + k]`, r ∈ 0..4, k ∈ 0..8 (7 observations + biais).
    pub const W_IN: usize = 0;
    /// `W_lat[r][l] = w[32 + 4 r + l]`, r, l ∈ 0..4.
    pub const W_LAT: usize = 32;
    /// `W_q[k] = w[48 + k]`, k ∈ 0..8 (4 sur h, 4 sur h2).
    pub const W_Q: usize = 48;
    /// `W_a[k] = w[56 + k]`, k ∈ 0..8.
    pub const W_A: usize = 56;
}

/// Observation : `(x0, x1, x2, x3, q, qv, qz)`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Obs {
    pub x: [u8; 4],
    pub q: u8,
    pub qv: u8,
    pub qz: u8,
}

impl Obs {
    /// `o0 = (x, 0, 0, 0)`.
    pub fn initial(x: [u8; 4]) -> Obs {
        Obs { x, q: 0, qv: 0, qz: 0 }
    }
    /// `o1 = (x, q, q·v, q·z)` : les valeurs cachées sont masquées par zéro sans enquête.
    pub fn final_obs(x: [u8; 4], q: u8, v: u8, z: u8) -> Obs {
        if q == 1 { Obs { x, q: 1, qv: v & 1, qz: z & 1 } } else { Obs { x, q: 0, qv: 0, qz: 0 } }
    }
    /// Depuis un vecteur de sept réels binaires.
    pub fn from_vector(v: &[f64; 7]) -> Obs {
        let b = |f: f64| if f >= 0.5 { 1u8 } else { 0u8 };
        Obs { x: [b(v[0]), b(v[1]), b(v[2]), b(v[3])], q: b(v[4]), qv: b(v[5]), qz: b(v[6]) }
    }
    /// Entrée du réseau : sept observations puis le biais 1.
    pub fn input(&self) -> [f64; INPUT_DIM] {
        [
            self.x[0] as f64,
            self.x[1] as f64,
            self.x[2] as f64,
            self.x[3] as f64,
            self.q as f64,
            self.qv as f64,
            self.qz as f64,
            1.0,
        ]
    }
}

/// Tête de décision.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Head {
    /// `q` : enquêter ou non, à partir de `o0`.
    Investigate,
    /// `a` : action finale, à partir de `o1`.
    Act,
}

impl Head {
    fn readout_offset(self) -> usize {
        match self {
            Head::Investigate => layout::W_Q,
            Head::Act => layout::W_A,
        }
    }
    fn gate_gene(self) -> usize {
        match self {
            Head::Investigate => gene::GATE_Q,
            Head::Act => gene::GATE_A,
        }
    }
}

/// Activations d'une passe avant, conservées pour le gradient et l'utilité.
#[derive(Clone, Debug, PartialEq)]
pub struct Forward {
    pub head: Head,
    pub input: [f64; INPUT_DIM],
    /// `s1 = W_in · input`.
    pub s1: [[f64; HIDDEN]; CELLS],
    /// `h = tanh(s1)` (zéro si la cellule est masquée).
    pub h: [[f64; HIDDEN]; CELLS],
    /// `m = (h_{c-1} + h_{c+1}) / 2`.
    pub m: [[f64; HIDDEN]; CELLS],
    /// `h2 = tanh(s1 + τ₂ W_lat m)` (zéro si masquée).
    pub h2: [[f64; HIDDEN]; CELLS],
    pub logit: f64,
    /// Probabilité brute `σ(logit)`, utilisée pour l'échantillonnage.
    pub p: f64,
    /// Probabilité bornée dans `[2^-16, 1 − 2^-16]`, utilisée pour les logarithmes et gradients.
    pub p_clamped: f64,
}

impl Forward {
    /// `log π(bit | o)` avec la probabilité bornée.
    pub fn log_prob(&self, bit: u8) -> f64 {
        if bit == 1 { self.p_clamped.ln() } else { (1.0 - self.p_clamped).ln() }
    }
}

/// Gradients des 1 024 poids.
pub type Grads = [[f64; WEIGHTS_PER_CELL]; CELLS];

/// Gradients par rapport aux activations `h` et `h2` de chaque cellule, pour une décision.
#[derive(Clone, Debug, PartialEq)]
pub struct ActGrads {
    pub h: [[f64; HIDDEN]; CELLS],
    pub h2: [[f64; HIDDEN]; CELLS],
}

impl ActGrads {
    pub fn zero() -> ActGrads {
        ActGrads { h: [[0.0; HIDDEN]; CELLS], h2: [[0.0; HIDDEN]; CELLS] }
    }
}

/// Gradients d'un épisode : poids (sommés sur les deux décisions) et activations par décision.
#[derive(Clone, Debug, PartialEq)]
pub struct EpisodeGrads {
    pub weights: Grads,
    pub act_q: ActGrads,
    pub act_a: ActGrads,
}

/// Résumé d'une mise à jour d'épisode.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EpisodeUpdate {
    /// `R − m` avec le `m` d'avant mise à jour.
    pub advantage: f64,
    /// `L = −(R − m)[log π(q|o0) + log π(a|o1)]`.
    pub loss: f64,
    /// Nouvelle référence `m`.
    pub baseline: f64,
}

/// Organisme : génome scellé, états réglementaires gelés, poids appris.
#[derive(Clone, Debug, PartialEq)]
pub struct Organism {
    genome: Genome,
    fingerprint: String,
    regulation: Regulation,
    weights: [[f64; WEIGHTS_PER_CELL]; CELLS],
    baseline: f64,
    utility: [f64; CELLS],
    masked: [bool; CELLS],
    uniform: bool,
    /// Compteur de temps : épisodes de vie ayant produit une mise à jour.
    age: u32,
    /// Présentations culturelles reçues.
    culture_presentations: u32,
}

/// Erreur de lecture d'un état sérialisé.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StateError {
    Format(String),
    /// L'empreinte du génome fourni ne correspond pas à celle enregistrée dans l'état.
    GenomeMismatch { expected: String, found: String },
}

impl std::fmt::Display for StateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StateError::Format(m) => write!(f, "état d'organisme invalide : {m}"),
            StateError::GenomeMismatch { expected, found } => {
                write!(f, "génome différent de celui de l'état : attendu {expected}, trouvé {found}")
            }
        }
    }
}

impl std::error::Error for StateError {}

/// Version du format d'état.
pub const STATE_FORMAT_VERSION: u32 = 1;

fn hex_f64(v: f64) -> String {
    format!("0x{:016x}", v.to_bits())
}

fn parse_hex_f64(s: &str) -> Result<f64, StateError> {
    let digits = s
        .strip_prefix("0x")
        .ok_or_else(|| StateError::Format(format!("réel sans préfixe 0x : {s:?}")))?;
    if digits.len() != 16 {
        return Err(StateError::Format(format!("réel de longueur incorrecte : {s:?}")));
    }
    let bits = u64::from_str_radix(digits, 16).map_err(|_| StateError::Format(format!("réel illisible : {s:?}")))?;
    let v = f64::from_bits(bits);
    if !v.is_finite() {
        return Err(StateError::Format(format!("réel non fini : {s:?}")));
    }
    Ok(v)
}

fn hex_list(values: impl Iterator<Item = f64>) -> String {
    let parts: Vec<String> = values.map(hex_f64).collect();
    format!("[{}]", parts.join(", "))
}

fn parse_list_items(value: &str, expected: usize) -> Result<Vec<&str>, StateError> {
    let inner = value
        .strip_prefix('[')
        .and_then(|v| v.strip_suffix(']'))
        .ok_or_else(|| StateError::Format("liste sans crochets".into()))?;
    let items: Vec<&str> = if inner.trim().is_empty() { Vec::new() } else { inner.split(',').map(|s| s.trim()).collect() };
    if items.len() != expected {
        return Err(StateError::Format(format!("liste de {} éléments au lieu de {expected}", items.len())));
    }
    Ok(items)
}

fn parse_hex_list(value: &str, expected: usize) -> Result<Vec<f64>, StateError> {
    parse_list_items(value, expected)?.into_iter().map(parse_hex_f64).collect()
}

impl Organism {
    pub fn age(&self) -> u32 {
        self.age
    }
    pub fn culture_presentations(&self) -> u32 {
        self.culture_presentations
    }

    /// État complet sérialisé, texte canonique et exact (réels en bits hexadécimaux) :
    /// génome (empreinte), mode, compteurs, référence, états réglementaires, transcriptomes,
    /// plasticités, poids, utilités, masques. Le génome lui-même est stocké à part
    /// (`genome.toml`) et vérifié à la relecture.
    pub fn to_state_string(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!("berceau-organism-state = {STATE_FORMAT_VERSION}\n"));
        s.push_str(&format!("lib = \"{}\"\n", crate::GENOME_LIB_VERSION));
        s.push_str(&format!("genome = \"{}\"\n", self.fingerprint));
        s.push_str(&format!("uniform = {}\n", self.uniform));
        s.push_str(&format!("age = {}\n", self.age));
        s.push_str(&format!("culture_presentations = {}\n", self.culture_presentations));
        s.push_str(&format!("baseline = {}\n", hex_f64(self.baseline)));
        s.push_str(&format!("regulation_state = {}\n", hex_list(self.regulation.state.iter().flatten().copied())));
        s.push_str(&format!("tau = {}\n", hex_list(self.regulation.tau.iter().flatten().copied())));
        s.push_str(&format!("eta = {}\n", hex_list(self.regulation.eta.iter().copied())));
        s.push_str(&format!("weights = {}\n", hex_list(self.weights.iter().flatten().copied())));
        s.push_str(&format!("utility = {}\n", hex_list(self.utility.iter().copied())));
        let masks: Vec<&str> = self.masked.iter().map(|m| if *m { "1" } else { "0" }).collect();
        s.push_str(&format!("masked = [{}]\n", masks.join(", ")));
        s
    }

    /// Empreinte SHA-256 du texte d'état.
    pub fn state_fingerprint(&self) -> String {
        crate::sha256::sha256_hex(self.to_state_string().as_bytes())
    }

    /// Relecture stricte d'un état : toutes les clés sont exigées une fois, dans n'importe
    /// quel ordre ; le génome fourni doit avoir l'empreinte enregistrée.
    pub fn from_state_string(text: &str, genome: Genome) -> Result<Organism, StateError> {
        use std::collections::HashMap;
        let mut map: HashMap<&str, &str> = HashMap::new();
        for (n, raw) in text.lines().enumerate() {
            let line = raw.trim();
            if line.is_empty() {
                continue;
            }
            let (k, v) = line
                .split_once('=')
                .ok_or_else(|| StateError::Format(format!("ligne {} sans '='", n + 1)))?;
            if map.insert(k.trim(), v.trim()).is_some() {
                return Err(StateError::Format(format!("clé répétée : {}", k.trim())));
            }
        }
        let get = |key: &str| map.get(key).copied().ok_or_else(|| StateError::Format(format!("clé absente : {key}")));
        let expected_keys = [
            "berceau-organism-state",
            "lib",
            "genome",
            "uniform",
            "age",
            "culture_presentations",
            "baseline",
            "regulation_state",
            "tau",
            "eta",
            "weights",
            "utility",
            "masked",
        ];
        for k in map.keys() {
            if !expected_keys.contains(k) {
                return Err(StateError::Format(format!("clé inconnue : {k}")));
            }
        }
        let version: u32 = get("berceau-organism-state")?.parse().map_err(|_| StateError::Format("version illisible".into()))?;
        if version != STATE_FORMAT_VERSION {
            return Err(StateError::Format(format!("version non prise en charge : {version}")));
        }
        let lib = get("lib")?.trim_matches('"');
        if lib != crate::GENOME_LIB_VERSION {
            return Err(StateError::Format(format!("bibliothèque différente : {lib}")));
        }
        let expected_fp = get("genome")?.trim_matches('"').to_string();
        let found_fp = genome.fingerprint();
        if expected_fp != found_fp {
            return Err(StateError::GenomeMismatch { expected: expected_fp, found: found_fp });
        }
        let uniform = match get("uniform")? {
            "true" => true,
            "false" => false,
            other => return Err(StateError::Format(format!("uniform illisible : {other}"))),
        };
        let age: u32 = get("age")?.parse().map_err(|_| StateError::Format("age illisible".into()))?;
        let culture_presentations: u32 =
            get("culture_presentations")?.parse().map_err(|_| StateError::Format("culture_presentations illisible".into()))?;
        let baseline = parse_hex_f64(get("baseline")?)?;

        let reg_state = parse_hex_list(get("regulation_state")?, CELLS * GENES)?;
        let tau = parse_hex_list(get("tau")?, CELLS * GENES)?;
        let eta = parse_hex_list(get("eta")?, CELLS)?;
        let weights = parse_hex_list(get("weights")?, CELLS * WEIGHTS_PER_CELL)?;
        let utility = parse_hex_list(get("utility")?, CELLS)?;
        let masks = parse_list_items(get("masked")?, CELLS)?;

        let mut regulation = Regulation { state: [[0.0; GENES]; CELLS], tau: [[0.0; GENES]; CELLS], eta: [0.0; CELLS] };
        let mut w = [[0.0; WEIGHTS_PER_CELL]; CELLS];
        let mut u = [0.0; CELLS];
        let mut masked = [false; CELLS];
        for c in 0..CELLS {
            for k in 0..GENES {
                regulation.state[c][k] = reg_state[c * GENES + k];
                regulation.tau[c][k] = tau[c * GENES + k];
                if !(0.0..=1.0).contains(&regulation.tau[c][k]) {
                    return Err(StateError::Format(format!("transcriptome hors [0,1] : {}", regulation.tau[c][k])));
                }
            }
            regulation.eta[c] = eta[c];
            if !(crate::develop::ETA_MIN - 1e-15..=crate::develop::ETA_MAX + 1e-15).contains(&eta[c]) {
                return Err(StateError::Format(format!("plasticité hors domaine : {}", eta[c])));
            }
            for k in 0..WEIGHTS_PER_CELL {
                w[c][k] = weights[c * WEIGHTS_PER_CELL + k];
                if w[c][k].abs() > WEIGHT_CLAMP {
                    return Err(StateError::Format(format!("poids hors [-8,8] : {}", w[c][k])));
                }
            }
            u[c] = utility[c];
            masked[c] = match masks[c] {
                "0" => false,
                "1" => true,
                other => return Err(StateError::Format(format!("masque illisible : {other}"))),
            };
        }
        Ok(Organism {
            genome,
            fingerprint: found_fp,
            regulation,
            weights: w,
            baseline,
            utility: u,
            masked,
            uniform,
            age,
            culture_presentations,
        })
    }
}

impl Organism {
    /// Développe un organisme : poids initiaux décodés du génome, puis 32 tours de
    /// régulation au flux `DEVELOPMENT` sous `base` (`g, i, j, phase`).
    /// `uniform = true` applique le témoin U (PROTOCOLE §3.3).
    pub fn develop(genome: Genome, rng: &Rng, base: Coord, uniform: bool) -> Organism {
        let params = RegParams::from_genome(&genome);
        let regulation = develop(&params, rng, base, uniform);
        let weights = decode_initial_weights(&genome);
        let fingerprint = genome.fingerprint();
        Organism {
            genome,
            fingerprint,
            regulation,
            weights,
            baseline: 0.0,
            utility: [0.0; CELLS],
            masked: [false; CELLS],
            uniform,
            age: 0,
            culture_presentations: 0,
        }
    }

    // ----- accès en lecture -----

    pub fn genome(&self) -> &Genome {
        &self.genome
    }
    /// Empreinte du génome, calculée à la naissance.
    pub fn fingerprint(&self) -> &str {
        &self.fingerprint
    }
    /// Vérifie que le génome porté correspond toujours à l'empreinte de naissance.
    pub fn verify_fingerprint(&self) -> bool {
        self.genome.fingerprint() == self.fingerprint
    }
    pub fn weights(&self) -> &[[f64; WEIGHTS_PER_CELL]; CELLS] {
        &self.weights
    }
    pub fn transcriptomes(&self) -> &[[f64; GENES]; CELLS] {
        &self.regulation.tau
    }
    pub fn plasticity(&self) -> &[f64; CELLS] {
        &self.regulation.eta
    }
    pub fn regulation(&self) -> &Regulation {
        &self.regulation
    }
    pub fn baseline(&self) -> f64 {
        self.baseline
    }
    pub fn utilities(&self) -> &[f64; CELLS] {
        &self.utility
    }
    pub fn is_uniform_control(&self) -> bool {
        self.uniform
    }
    pub fn masked(&self) -> &[bool; CELLS] {
        &self.masked
    }

    /// Variance moyenne des transcriptomes sur les 16 cellules (diagnostic C2).
    pub fn transcriptome_variance(&self) -> f64 {
        let tau = &self.regulation.tau;
        (0..GENES)
            .map(|k| {
                let mean = tau.iter().map(|t| t[k]).sum::<f64>() / CELLS as f64;
                tau.iter().map(|t| (t[k] - mean).powi(2)).sum::<f64>() / CELLS as f64
            })
            .sum::<f64>()
            / GENES as f64
    }

    // ----- écritures contrôlées -----

    /// Fixe un poids (diagnostics et tests de gradient) ; borné dans `[−8, 8]`.
    pub fn set_weight(&mut self, cell: usize, k: usize, value: f64) {
        self.weights[cell][k] = value.clamp(-WEIGHT_CLAMP, WEIGHT_CLAMP);
    }

    /// Masque ou démasque la sortie d'une cellule (contrôle mécanistique H2).
    pub fn set_mask(&mut self, cell: usize, masked: bool) {
        self.masked[cell] = masked;
    }
    pub fn clear_masks(&mut self) {
        self.masked = [false; CELLS];
    }

    /// Copie les 64 poids de la cellule `cell` du donneur ; rien d'autre n'est copié.
    pub fn copy_cell_weights(&mut self, donor: &Organism, cell: usize) {
        self.weights[cell] = donor.weights[cell];
    }

    /// Transfert total `T` : les 1 024 poids du donneur.
    pub fn transfer_all(&mut self, donor: &Organism) {
        for c in 0..CELLS {
            self.copy_cell_weights(donor, c);
        }
    }

    /// Transfert partiel `P` : huit cellules choisies par permutation aléatoire du
    /// tirage `draw` (Fisher–Yates avec `uniform_usize`), indépendante des activités.
    /// Rend les cellules copiées, dans l'ordre croissant.
    pub fn transfer_partial(&mut self, donor: &Organism, draw: &mut Draw) -> Vec<usize> {
        let mut perm: Vec<usize> = (0..CELLS).collect();
        for i in (1..CELLS).rev() {
            let j = draw.uniform_usize(i + 1);
            perm.swap(i, j);
        }
        let mut chosen: Vec<usize> = perm[..PARTIAL_TRANSFER_CELLS].to_vec();
        chosen.sort_unstable();
        for &c in &chosen {
            self.copy_cell_weights(donor, c);
        }
        chosen
    }

    /// Réinitialise les 64 poids de `cell` indépendamment dans `[−1/20, 1/20]`
    /// (`w = (2U − 1)/20`), remet son utilité à zéro ; génome et transcriptome conservés.
    pub fn renew_cell(&mut self, cell: usize, draw: &mut Draw) {
        for w in self.weights[cell].iter_mut() {
            *w = (2.0 * draw.uniform_f64() - 1.0) / INITIAL_WEIGHT_DIVISOR;
        }
        self.utility[cell] = 0.0;
    }

    /// Cellule d'utilité minimale, égalité départagée par l'identifiant le plus petit.
    pub fn least_useful_cell(&self) -> usize {
        let mut best = 0;
        for c in 1..CELLS {
            if self.utility[c] < self.utility[best] {
                best = c;
            }
        }
        best
    }

    /// Renouvelle la cellule d'utilité minimale et rend son indice.
    pub fn renew_least_useful(&mut self, draw: &mut Draw) -> usize {
        let c = self.least_useful_cell();
        self.renew_cell(c, draw);
        c
    }

    /// Vrai aux épisodes `31, 63, …, 255` : `t mod 32 == 31`.
    pub fn is_renewal_episode(t: u32) -> bool {
        t % 32 == 31
    }

    // ----- passe avant -----

    /// Passe avant pour une tête.
    pub fn forward(&self, obs: &Obs, head: Head) -> Forward {
        let input = obs.input();
        let mut s1 = [[0.0; HIDDEN]; CELLS];
        let mut h = [[0.0; HIDDEN]; CELLS];
        for c in 0..CELLS {
            let w = &self.weights[c];
            for r in 0..HIDDEN {
                let mut s = 0.0;
                for k in 0..INPUT_DIM {
                    s += w[layout::W_IN + r * INPUT_DIM + k] * input[k];
                }
                s1[c][r] = s;
                h[c][r] = if self.masked[c] { 0.0 } else { s.tanh() };
            }
        }
        let mut m = [[0.0; HIDDEN]; CELLS];
        let mut h2 = [[0.0; HIDDEN]; CELLS];
        for c in 0..CELLS {
            let left = h[(c + CELLS - 1) % CELLS];
            let right = h[(c + 1) % CELLS];
            for l in 0..HIDDEN {
                m[c][l] = 0.5 * (left[l] + right[l]);
            }
            let lat = self.regulation.tau[c][gene::LATERAL];
            let w = &self.weights[c];
            for r in 0..HIDDEN {
                let mut s = s1[c][r];
                for l in 0..HIDDEN {
                    s += lat * w[layout::W_LAT + r * HIDDEN + l] * m[c][l];
                }
                h2[c][r] = if self.masked[c] { 0.0 } else { s.tanh() };
            }
        }
        let off = head.readout_offset();
        let gene_idx = head.gate_gene();
        let mut logit = 0.0;
        for c in 0..CELLS {
            if self.masked[c] {
                continue;
            }
            let gate = self.regulation.tau[c][gene_idx];
            let w = &self.weights[c];
            let mut s = 0.0;
            for k in 0..HIDDEN {
                s += w[off + k] * h[c][k] + w[off + HIDDEN + k] * h2[c][k];
            }
            logit += gate * s;
        }
        let p = 1.0 / (1.0 + (-logit).exp());
        let p_clamped = p.clamp(PROB_EPS, 1.0 - PROB_EPS);
        Forward { head, input, s1, h, m, h2, logit, p, p_clamped }
    }

    /// `p(bit = 1 | obs)` pour la tête donnée (probabilité brute).
    pub fn probability(&self, obs: &Obs, head: Head) -> f64 {
        self.forward(obs, head).p
    }

    /// Échantillonne une décision avec l'uniforme `u` : `1` si `u < p`.
    pub fn sample(&self, obs: &Obs, head: Head, u: f64) -> (u8, Forward) {
        let f = self.forward(obs, head);
        let bit = if u < f.p { 1 } else { 0 };
        (bit, f)
    }

    // ----- rétropropagation -----

    /// Accumule dans `grads` le gradient de `g_logit · logit` et dans `act` les gradients
    /// par rapport à `h` et `h2`. `g_logit = ∂L/∂logit`.
    pub fn backward(&self, f: &Forward, g_logit: f64, grads: &mut Grads, act: &mut ActGrads) {
        let off = f.head.readout_offset();
        let gene_idx = f.head.gate_gene();
        let mut gh = [[0.0; HIDDEN]; CELLS];
        let mut gh2 = [[0.0; HIDDEN]; CELLS];
        // lecture
        for c in 0..CELLS {
            if self.masked[c] {
                continue;
            }
            let gate = self.regulation.tau[c][gene_idx];
            let w = &self.weights[c];
            for k in 0..HIDDEN {
                grads[c][off + k] += g_logit * gate * f.h[c][k];
                grads[c][off + HIDDEN + k] += g_logit * gate * f.h2[c][k];
                gh[c][k] += g_logit * gate * w[off + k];
                gh2[c][k] += g_logit * gate * w[off + HIDDEN + k];
            }
        }
        // seconde passe : h2 = tanh(s1 + lat · W_lat m)
        let mut gs2 = [[0.0; HIDDEN]; CELLS];
        for c in 0..CELLS {
            if self.masked[c] {
                continue;
            }
            let lat = self.regulation.tau[c][gene::LATERAL];
            let w = &self.weights[c];
            for r in 0..HIDDEN {
                gs2[c][r] = gh2[c][r] * (1.0 - f.h2[c][r] * f.h2[c][r]);
            }
            let mut gm = [0.0; HIDDEN];
            for r in 0..HIDDEN {
                for l in 0..HIDDEN {
                    grads[c][layout::W_LAT + r * HIDDEN + l] += gs2[c][r] * lat * f.m[c][l];
                    gm[l] += gs2[c][r] * lat * w[layout::W_LAT + r * HIDDEN + l];
                }
                for k in 0..INPUT_DIM {
                    grads[c][layout::W_IN + r * INPUT_DIM + k] += gs2[c][r] * f.input[k];
                }
            }
            let left = (c + CELLS - 1) % CELLS;
            let right = (c + 1) % CELLS;
            for l in 0..HIDDEN {
                gh[left][l] += 0.5 * gm[l];
                gh[right][l] += 0.5 * gm[l];
            }
        }
        // première passe : h = tanh(s1)
        for c in 0..CELLS {
            if self.masked[c] {
                continue;
            }
            for r in 0..HIDDEN {
                let gs1 = gh[c][r] * (1.0 - f.h[c][r] * f.h[c][r]);
                for k in 0..INPUT_DIM {
                    grads[c][layout::W_IN + r * INPUT_DIM + k] += gs1 * f.input[k];
                }
            }
        }
        // Une cellule masquée a des sorties constantes (zéro) : aucun gradient d'activation.
        for c in 0..CELLS {
            if self.masked[c] {
                continue;
            }
            for r in 0..HIDDEN {
                act.h[c][r] += gh[c][r];
                act.h2[c][r] += gh2[c][r];
            }
        }
    }

    /// Gradients d'un épisode complet pour l'avantage `adv = R − m` :
    /// `∂L/∂logit_q = −adv (q − p̃_q)`, `∂L/∂logit_a = −adv (a − p̃_a)`.
    /// Les gradients de poids des deux décisions sont sommés ; les gradients d'activation
    /// sont rendus **séparément** pour chaque décision (`act_q`, `act_a`), afin que
    /// l'utilité n'introduise aucun terme croisé entre les deux passes.
    pub fn episode_gradients(&self, fq: &Forward, q: u8, fa: &Forward, a: u8, advantage: f64) -> EpisodeGrads {
        let mut weights = [[0.0; WEIGHTS_PER_CELL]; CELLS];
        let mut act_q = ActGrads::zero();
        let mut act_a = ActGrads::zero();
        let gq = -advantage * (q as f64 - fq.p_clamped);
        let ga = -advantage * (a as f64 - fa.p_clamped);
        self.backward(fq, gq, &mut weights, &mut act_q);
        self.backward(fa, ga, &mut weights, &mut act_a);
        EpisodeGrads { weights, act_q, act_a }
    }

    /// Perte d'épisode `L = −(R − m)[log π(q|o0) + log π(a|o1)]` pour la référence courante.
    pub fn episode_loss(&self, fq: &Forward, q: u8, fa: &Forward, a: u8, reward: f64) -> f64 {
        -(reward - self.baseline) * (fq.log_prob(q) + fa.log_prob(a))
    }

    /// Mise à jour unique de l'épisode (PROTOCOLE §3.2) : descente locale `w ← w − η_c ∂L/∂w`
    /// bornée dans `[−8, 8]`, puis `m ← (15 m + R)/16` et mise à jour des utilités
    /// `u ← 0,99 u + 0,01 mean(|act · ∂L/∂act|)` sur les 16 activations de la cellule.
    pub fn learn_episode(&mut self, fq: &Forward, q: u8, fa: &Forward, a: u8, reward: f64) -> EpisodeUpdate {
        let advantage = reward - self.baseline;
        let loss = self.episode_loss(fq, q, fa, a, reward);
        let g = self.episode_gradients(fq, q, fa, a, advantage);
        for c in 0..CELLS {
            let eta = self.regulation.eta[c];
            for k in 0..WEIGHTS_PER_CELL {
                self.weights[c][k] = (self.weights[c][k] - eta * g.weights[c][k]).clamp(-WEIGHT_CLAMP, WEIGHT_CLAMP);
            }
            // Moyenne des 16 produits |activation × gradient| de la cellule : pour chaque
            // décision, ses 4 h et ses 4 h2 avec les gradients de CETTE décision (aucun terme croisé).
            let mut acc = 0.0;
            for r in 0..HIDDEN {
                acc += (fq.h[c][r] * g.act_q.h[c][r]).abs() + (fq.h2[c][r] * g.act_q.h2[c][r]).abs();
                acc += (fa.h[c][r] * g.act_a.h[c][r]).abs() + (fa.h2[c][r] * g.act_a.h2[c][r]).abs();
            }
            self.utility[c] = UTILITY_KEEP * self.utility[c] + (1.0 - UTILITY_KEEP) * acc / (4.0 * HIDDEN as f64);
        }
        self.baseline = BASELINE_KEEP * self.baseline + (1.0 - BASELINE_KEEP) * reward;
        self.age += 1;
        EpisodeUpdate { advantage, loss, baseline: self.baseline }
    }

    /// Perte culturelle d'un enregistrement : `−log π(q_prof|o0) − log π(a_prof|o1_prof)`.
    pub fn culture_loss(&self, rec: &Record) -> f64 {
        let fq = self.forward(&rec.obs0(), Head::Investigate);
        let fa = self.forward(&rec.obs1(), Head::Act);
        -(fq.log_prob(rec.q) + fa.log_prob(rec.a))
    }

    /// Une présentation culturelle (PROTOCOLE §4.2) : descente à taux fixe `1/64`, commune à
    /// toutes les cellules, sur `L_culture` ; poids bornés ; `m` et utilités inchangés.
    /// Rend la perte avant mise à jour.
    pub fn learn_record(&mut self, rec: &Record) -> f64 {
        let fq = self.forward(&rec.obs0(), Head::Investigate);
        let fa = self.forward(&rec.obs1(), Head::Act);
        let loss = -(fq.log_prob(rec.q) + fa.log_prob(rec.a));
        let mut grads = [[0.0; WEIGHTS_PER_CELL]; CELLS];
        let mut act = ActGrads::zero();
        self.backward(&fq, -(rec.q as f64 - fq.p_clamped), &mut grads, &mut act);
        self.backward(&fa, -(rec.a as f64 - fa.p_clamped), &mut grads, &mut act);
        for c in 0..CELLS {
            for k in 0..WEIGHTS_PER_CELL {
                self.weights[c][k] = (self.weights[c][k] - CULTURE_RATE * grads[c][k]).clamp(-WEIGHT_CLAMP, WEIGHT_CLAMP);
            }
        }
        self.culture_presentations += 1;
        loss
    }

    /// Éducation complète : quatre passages sur les 32 enregistrements dans l'ordre canonique
    /// (128 présentations). Rend la perte moyenne de chaque passage.
    pub fn educate(&mut self, packet: &Packet) -> [f64; CULTURE_PASSES] {
        let mut out = [0.0; CULTURE_PASSES];
        for (pass, slot) in out.iter_mut().enumerate() {
            let mut total = 0.0;
            for rec in packet.records().iter() {
                total += self.learn_record(rec);
            }
            *slot = total / packet.records().len() as f64;
            let _ = pass;
        }
        out
    }

    /// Écart aux démonstrations (diagnostic secondaire, PROTOCOLE §6.1) :
    /// moyenne de `|p(q=1|o0) − q_prof|` et `|p(a=1|o1_prof) − a_prof|` sur le paquet.
    pub fn culture_gap(&self, packet: &Packet) -> f64 {
        let mut total = 0.0;
        for rec in packet.records().iter() {
            total += (self.probability(&rec.obs0(), Head::Investigate) - rec.q as f64).abs();
            total += (self.probability(&rec.obs1(), Head::Act) - rec.a as f64).abs();
        }
        total / (2.0 * packet.records().len() as f64)
    }

    /// Carte cellulaire textuelle : par cellule, `τ₀ τ₁ τ₂ τ₃ η u masque`, une ligne par cellule.
    pub fn cell_map(&self) -> String {
        let mut s = String::new();
        for c in 0..CELLS {
            let t = self.regulation.tau[c];
            s.push_str(&format!(
                "{c:2} tau=[{:.4} {:.4} {:.4} {:.4}] eta={:.6} u={:.6}{}\n",
                t[0],
                t[1],
                t[2],
                t[3],
                self.regulation.eta[c],
                self.utility[c],
                if self.masked[c] { " masked" } else { "" }
            ));
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::genome::EncodingKind;

    fn organism(root: u64, kind: EncodingKind) -> Organism {
        let rng = Rng::new(root);
        let g = Genome::random(kind, &rng, Coord::default());
        Organism::develop(g, &rng, Coord::default(), false)
    }

    #[test]
    fn probabilities_are_bounded_and_heads_differ() {
        let o = organism(1, EncodingKind::Indirect);
        let obs = Obs::initial([1, 0, 1, 0]);
        let fq = o.forward(&obs, Head::Investigate);
        let fa = o.forward(&obs, Head::Act);
        assert!(fq.p > 0.0 && fq.p < 1.0);
        assert!(fq.p_clamped >= PROB_EPS && fq.p_clamped <= 1.0 - PROB_EPS);
        assert_ne!(fq.logit, fa.logit, "les deux têtes lisent des poids distincts");
    }

    #[test]
    fn sampling_follows_uniform() {
        let o = organism(1, EncodingKind::Indirect);
        let obs = Obs::initial([0, 0, 0, 0]);
        let p = o.probability(&obs, Head::Act);
        assert_eq!(o.sample(&obs, Head::Act, p - 1e-9).0, 1);
        assert_eq!(o.sample(&obs, Head::Act, p + 1e-9).0, 0);
    }

    #[test]
    fn weights_stay_bounded_and_baseline_moves() {
        let mut o = organism(2, EncodingKind::Direct);
        for c in 0..CELLS {
            for k in 0..WEIGHTS_PER_CELL {
                o.set_weight(c, k, 7.9);
            }
        }
        let obs0 = Obs::initial([1, 1, 0, 0]);
        let obs1 = Obs::final_obs([1, 1, 0, 0], 1, 1, 0);
        for _ in 0..50 {
            let (q, fq) = o.sample(&obs0, Head::Investigate, 0.3);
            let (a, fa) = o.sample(&obs1, Head::Act, 0.6);
            o.learn_episode(&fq, q, &fa, a, 1.0);
        }
        for c in 0..CELLS {
            for k in 0..WEIGHTS_PER_CELL {
                assert!(o.weights()[c][k].abs() <= WEIGHT_CLAMP);
            }
        }
        assert!((o.baseline() - (1.0 - (15.0f64 / 16.0).powi(50))).abs() < 1e-12);
    }

    #[test]
    fn culture_reduces_loss_and_leaves_baseline() {
        let mut o = organism(3, EncodingKind::Indirect);
        let mut recs = [Record::default(); 32];
        for (i, r) in recs.iter_mut().enumerate() {
            let b = |n: usize| ((i >> n) & 1) as u8;
            let x = [b(0), b(1), b(2), 0];
            let (v, z) = (b(3), b(4));
            let bq = x[0] ^ x[1];
            *r = Record::from_experience(x, 1, v, z, bq, 1, v & ((bq == z) as u8));
        }
        let packet = Packet::from_records(recs);
        let before: f64 = packet.records().iter().map(|r| o.culture_loss(r)).sum();
        let losses = o.educate(&packet);
        let after: f64 = packet.records().iter().map(|r| o.culture_loss(r)).sum();
        assert!(after < before, "avant {before} après {after}");
        assert!(losses[3] < losses[0]);
        assert_eq!(o.baseline(), 0.0);
        assert_eq!(o.utilities(), &[0.0; CELLS]);
    }

    #[test]
    fn masked_cell_contributes_nothing_and_gets_no_gradient() {
        let mut o = organism(4, EncodingKind::Indirect);
        o.set_mask(5, true);
        let obs = Obs::initial([1, 0, 1, 1]);
        let f = o.forward(&obs, Head::Act);
        assert_eq!(f.h[5], [0.0; HIDDEN]);
        assert_eq!(f.h2[5], [0.0; HIDDEN]);
        let fq = o.forward(&obs, Head::Investigate);
        let g = o.episode_gradients(&fq, 1, &f, 0, 0.5);
        assert_eq!(g.weights[5], [0.0; WEIGHTS_PER_CELL]);
        assert!(g.weights[4].iter().any(|v| *v != 0.0));
        assert_eq!(g.act_q.h[5], [0.0; HIDDEN]);
        assert_eq!(g.act_a.h2[5], [0.0; HIDDEN]);
        o.clear_masks();
        assert_eq!(o.masked(), &[false; CELLS]);
    }

    #[test]
    fn renewal_and_transfers() {
        let rng = Rng::new(5);
        let g = Genome::random(EncodingKind::Indirect, &rng, Coord::default());
        let donor = Organism::develop(g.clone(), &rng, Coord::default(), false);
        let mut student = Organism::develop(g, &rng, Coord::default(), false);
        for c in 0..CELLS {
            for k in 0..WEIGHTS_PER_CELL {
                student.set_weight(c, k, 1.0);
            }
        }
        let mut d = rng.draw(Coord::of_stream(8));
        let chosen = student.transfer_partial(&donor, &mut d);
        assert_eq!(chosen.len(), PARTIAL_TRANSFER_CELLS);
        for c in 0..CELLS {
            if chosen.contains(&c) {
                assert_eq!(student.weights()[c], donor.weights()[c]);
            } else {
                assert_eq!(student.weights()[c], [1.0; WEIGHTS_PER_CELL]);
            }
        }
        student.transfer_all(&donor);
        assert_eq!(student.weights(), donor.weights());
        assert_eq!(student.transcriptomes(), donor.transcriptomes());

        // utilités nulles : égalité → cellule 0
        assert_eq!(student.least_useful_cell(), 0);
        let mut d2 = rng.draw(Coord::of_stream(8).with_t(31));
        let renewed = student.renew_least_useful(&mut d2);
        assert_eq!(renewed, 0);
        for &w in student.weights()[0].iter() {
            assert!(w.abs() <= INITIAL_WEIGHT_BOUND);
        }
        assert!(Organism::is_renewal_episode(31) && Organism::is_renewal_episode(255));
        assert!(!Organism::is_renewal_episode(32) && !Organism::is_renewal_episode(0));
    }

    #[test]
    fn utility_uses_per_decision_products_without_cross_terms() {
        let mut o = organism(9, EncodingKind::Indirect);
        let w0 = *o.weights();
        for c in 0..CELLS {
            for k in 0..WEIGHTS_PER_CELL {
                o.set_weight(c, k, w0[c][k] * 8.0);
            }
        }
        let o0 = Obs::initial([1, 0, 1, 0]);
        let o1 = Obs::final_obs([1, 0, 1, 0], 1, 1, 0);
        let fq = o.forward(&o0, Head::Investigate);
        let fa = o.forward(&o1, Head::Act);
        let (q, a, reward) = (1u8, 0u8, 0.8);
        let g = o.episode_gradients(&fq, q, &fa, a, reward - o.baseline());
        let mut expected = [0.0; CELLS];
        for c in 0..CELLS {
            let mut acc = 0.0;
            for r in 0..HIDDEN {
                acc += (fq.h[c][r] * g.act_q.h[c][r]).abs() + (fq.h2[c][r] * g.act_q.h2[c][r]).abs();
                acc += (fa.h[c][r] * g.act_a.h[c][r]).abs() + (fa.h2[c][r] * g.act_a.h2[c][r]).abs();
            }
            expected[c] = 0.01 * acc / 16.0;
        }
        o.learn_episode(&fq, q, &fa, a, reward);
        for c in 0..CELLS {
            assert!((o.utilities()[c] - expected[c]).abs() < 1e-15, "cellule {c}");
        }
        // les gradients d'activation des deux décisions diffèrent : une version croisée donnerait autre chose
        assert_ne!(g.act_q, g.act_a);
    }

    #[test]
    fn state_roundtrip_is_exact_and_strict() {
        let mut o = organism(7, EncodingKind::Indirect);
        let obs0 = Obs::initial([1, 0, 1, 0]);
        let obs1 = Obs::final_obs([1, 0, 1, 0], 1, 1, 1);
        for i in 0..5 {
            let (q, fq) = o.sample(&obs0, Head::Investigate, 0.4);
            let (a, fa) = o.sample(&obs1, Head::Act, 0.7);
            o.learn_episode(&fq, q, &fa, a, (i % 2) as f64);
        }
        o.set_mask(3, true);
        let rec = Record::from_experience([1, 1, 0, 0], 1, 0, 1, 0, 1, 0);
        o.learn_record(&rec);
        assert_eq!(o.age(), 5);
        assert_eq!(o.culture_presentations(), 1);

        let text = o.to_state_string();
        let back = Organism::from_state_string(&text, o.genome().clone()).unwrap();
        assert_eq!(back, o);
        assert_eq!(back.to_state_string(), text);
        assert_eq!(back.state_fingerprint(), o.state_fingerprint());

        // génome différent → refus
        let other = Genome::random(EncodingKind::Indirect, &Rng::new(99), Coord::default());
        assert!(matches!(Organism::from_state_string(&text, other), Err(StateError::GenomeMismatch { .. })));
        // clé manquante, clé inconnue, poids hors borne → refus
        let missing = text.lines().filter(|l| !l.starts_with("eta")).collect::<Vec<_>>().join("\n");
        assert!(matches!(Organism::from_state_string(&missing, o.genome().clone()), Err(StateError::Format(_))));
        let extra = format!("{text}extra = 1\n");
        assert!(matches!(Organism::from_state_string(&extra, o.genome().clone()), Err(StateError::Format(_))));
        let bad = text.replace(&format!("baseline = {}", hex_f64(o.baseline())), "baseline = 0x7ff0000000000000");
        assert!(matches!(Organism::from_state_string(&bad, o.genome().clone()), Err(StateError::Format(_))));
    }

    #[test]
    fn fingerprint_is_sealed_at_birth() {
        let o = organism(6, EncodingKind::Direct);
        assert!(o.verify_fingerprint());
        assert_eq!(o.fingerprint(), o.genome().fingerprint());
    }
}
