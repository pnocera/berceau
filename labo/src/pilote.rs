//! Pilote exploratoire `v2` (`labo/PROTOCOLE_PILOTE_V2.md`).
//!
//! Ce module n'altère pas `genome v1` : il réutilise ses passes avant et arrière publiques
//! (`Organism::forward`, `Organism::backward`) et applique lui-même la mise à jour plastique
//! (`w ← clamp(w − η_c g, −8, 8)` via `Organism::set_weight`), la ligne de base `m` et
//! l'utilité, pour pouvoir changer le seul facteur étudié de chaque case :
//!
//! * facteur `W` : diviseur `d ∈ {20, 2}` des poids initiaux indirects (§4.1) ;
//! * facteur `L` : `δ_a` REINFORCE (`L0`) ou entropie croisée sur la cible reconstruite
//!   `b` (`L1`), la tête `q` restant au renforcement sur `R − m` (§4.2).
//!
//! Sous `(W0, L0)`, la mise à jour reproduit `Organism::learn_episode` bit à bit (test).

use crate::{bit, Address, Result, World, LIFE};
use genome::{
    layout, word_to_unit, ActGrads, Coord, EncodingKind, Forward, Genome, Head, Obs, Organism, Rng, BASELINE_KEEP, CELLS, HIDDEN,
    UTILITY_KEEP, WEIGHTS_PER_CELL,
};

/// Racines d'exploration (§2).
pub const EXPLORATION_ROOTS: std::ops::Range<u64> = 100..116;
/// Racines de validation réservée (§2).
pub const VALIDATION_ROOTS: std::ops::Range<u64> = 116..132;
/// Racines techniques du test de fumée : hors de toutes les plages réservées.
pub const SMOKE_EXPLORATION_ROOTS: std::ops::Range<u64> = 900..916;
pub const SMOKE_VALIDATION_ROOTS: std::ops::Range<u64> = 916..932;
/// Répétitions indépendantes par case.
pub const REPLICATES: usize = 16;
/// Sondes sans apprentissage de chaque barre.
pub const PROBES: usize = 64;
/// Seuil commun des deux barres (§5.3, §5.4).
pub const THRESHOLD: f64 = 0.55;
/// Rééchantillonnages bootstrap (§5.2).
pub const PILOT_BOOTSTRAPS: usize = 100_000;
/// Rang de la borne inférieure unilatérale à 95 % (indexation `0..99_999`).
pub const LOW_RANK: usize = 5_000;
/// Flot dédié au bootstrap du pilote (§5.1).
pub const BOOTSTRAP_STREAM: u32 = 19;
/// Seuil du témoin statique de l'Étape 0 (§3).
pub const STATIC_THRESHOLD: f64 = 0.80;

/// Coordonnées du pilote (§5.1).
pub mod phase {
    pub const GENOME: u32 = 1;
    pub const DEVELOPMENT: u32 = 3;
    pub const LOW_LIFE: u32 = 11;
    pub const LOW_PROBES: u32 = 12;
    pub const HIGH_LIFE: u32 = 13;
    pub const HIGH_PROBES: u32 = 14;
}

/// Facteur `W` : échelle initiale des poids.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scale {
    W0,
    W1,
}
impl Scale {
    pub fn divisor(self) -> f64 {
        match self {
            Scale::W0 => 20.0,
            Scale::W1 => 2.0,
        }
    }
}

/// Facteur `L` : règle de mise à jour de la tête `a`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rule {
    L0,
    L1,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Case {
    pub scale: Scale,
    pub rule: Rule,
}
impl std::fmt::Display for Case {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?},{:?})", self.scale, self.rule)
    }
}
/// Ordre de priorité préenregistré (§6.3), qui est aussi l'ordre d'exécution.
pub const CASES: [Case; 4] = [
    Case { scale: Scale::W0, rule: Rule::L0 },
    Case { scale: Scale::W1, rule: Rule::L0 },
    Case { scale: Scale::W0, rule: Rule::L1 },
    Case { scale: Scale::W1, rule: Rule::L1 },
];

// ---------------------------------------------------------------------------
// Naissance
// ---------------------------------------------------------------------------

/// Génome initial (`stream = 1, g = i = j = 0, phase = 1, draw = 0..191`).
pub fn initial_genome(root: u64) -> Genome {
    Genome::random(
        EncodingKind::Indirect,
        &Rng::new(root),
        Coord { stream: 1, phase: phase::GENOME, ..Coord::default() },
    )
}

/// Poids initiaux indirects `tanh(2(f0 + f1 cos(θ_c + φ_k))) / d` (`SPEC_GENOME.md` §3.2),
/// écrits exactement comme `genome::decode_initial_weights` mais avec le diviseur `d`.
pub fn decode_weights(genome: &Genome, divisor: f64) -> Result<[[f64; WEIGHTS_PER_CELL]; CELLS]> {
    if genome.kind() != EncodingKind::Indirect {
        return Err("le pilote n'utilise que des génomes indirects (condition B)".into());
    }
    let e = genome.encoding();
    let mut w = [[0.0; WEIGHTS_PER_CELL]; CELLS];
    for k in 0..WEIGHTS_PER_CELL {
        let f0 = word_to_unit(e[2 * k]);
        let f1 = word_to_unit(e[2 * k + 1]);
        let phi = std::f64::consts::PI * (k as f64) / 32.0;
        for (c, cell) in w.iter_mut().enumerate() {
            let theta = 2.0 * std::f64::consts::PI * (c as f64) / (CELLS as f64);
            cell[k] = (2.0 * (f0 + f1 * (theta + phi).cos())).tanh() / divisor;
        }
    }
    Ok(w)
}

/// Organisme neuf (`age = 0, m = 0, u = 0`) : développement `stream = 2, phase = 3`,
/// puis poids initiaux de l'échelle de la case. Rien d'autre ne dépend de la case.
pub fn born(genome: &Genome, root: u64, scale: Scale) -> Result<Learner> {
    let mut organism =
        Organism::develop(genome.clone(), &Rng::new(root), Coord { phase: phase::DEVELOPMENT, ..Coord::default() }, false);
    if scale != Scale::W0 {
        let w = decode_weights(genome, scale.divisor())?;
        for (c, cell) in w.iter().enumerate() {
            for (k, &value) in cell.iter().enumerate() {
                organism.set_weight(c, k, value);
            }
        }
    }
    Ok(Learner { organism, baseline: 0.0, utility: [0.0; CELLS], updates: 0 })
}

// ---------------------------------------------------------------------------
// Apprentissage
// ---------------------------------------------------------------------------

/// Organisme plus l'état d'apprentissage que le pilote tient lui-même (`m`, utilités).
/// Les champs `baseline`, `utilities` et `age` de l'`Organism` embarqué restent à leur
/// valeur de naissance : seuls ses poids évoluent. Ne pas sérialiser cet organisme ni
/// appeler ses méthodes d'apprentissage ou de renouvellement (relecture P8 de Codex, F2).
#[derive(Clone, Debug)]
pub struct Learner {
    pub organism: Organism,
    pub baseline: f64,
    pub utility: [f64; CELLS],
    pub updates: u32,
}

impl Learner {
    /// Unique mise à jour d'épisode : gradients des deux têtes sommés `1:1` (dans l'ordre
    /// `q` puis `a`, comme `episode_gradients`), descente au taux local `η_c`, bornes `±8`,
    /// utilité `SPEC_GENOME.md` §6.3, puis `m ← 15/16 m + R/16`.
    /// `q = None` : aucune passe arrière sur la tête `q` (`δ_q = 0`, barre basse).
    pub fn update(&mut self, q: Option<(&Forward, f64)>, fa: &Forward, delta_a: f64, reward: f64) -> Result<()> {
        let org = &self.organism;
        let mut grads = [[0.0; WEIGHTS_PER_CELL]; CELLS];
        let mut act_q = ActGrads::zero();
        let mut act_a = ActGrads::zero();
        if let Some((fq, delta_q)) = q {
            org.backward(fq, delta_q, &mut grads, &mut act_q);
        }
        org.backward(fa, delta_a, &mut grads, &mut act_a);
        if grads.iter().flatten().any(|g| !g.is_finite()) {
            return Err("gradient non fini".into());
        }
        let eta = *org.plasticity();
        let mut weights = *org.weights();
        for c in 0..CELLS {
            for k in 0..WEIGHTS_PER_CELL {
                weights[c][k] -= eta[c] * grads[c][k];
            }
            let mut acc = 0.0;
            for r in 0..HIDDEN {
                if let Some((fq, _)) = q {
                    acc += (fq.h[c][r] * act_q.h[c][r]).abs() + (fq.h2[c][r] * act_q.h2[c][r]).abs();
                }
                acc += (fa.h[c][r] * act_a.h[c][r]).abs() + (fa.h2[c][r] * act_a.h2[c][r]).abs();
            }
            self.utility[c] = UTILITY_KEEP * self.utility[c] + (1.0 - UTILITY_KEEP) * acc / (4.0 * HIDDEN as f64);
        }
        for (c, cell) in weights.iter().enumerate() {
            for (k, &value) in cell.iter().enumerate() {
                self.organism.set_weight(c, k, value); // borne ±8 (WEIGHT_CLAMP)
            }
        }
        self.baseline = BASELINE_KEEP * self.baseline + (1.0 - BASELINE_KEEP) * reward;
        self.updates += 1;
        Ok(())
    }

    fn decide(&self, observation: &[f64; 7], head: Head, u: f64) -> Result<(bool, Forward)> {
        let f = self.organism.forward(&Obs::from_vector(observation), head);
        if !f.p.is_finite() || !(0.0..=1.0).contains(&f.p) {
            return Err("probabilité invalide".into());
        }
        Ok((u < f.p, f))
    }
}

// ---------------------------------------------------------------------------
// Barres de capacité
// ---------------------------------------------------------------------------

/// Résultats d'une répétition `(root, case)` : deux organismes neufs indépendants.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Replicate {
    pub root: u64,
    /// Barre basse : `Ret_aux` sur 64 sondes (`v = 1, q = 1`, cible `1 − z`).
    pub ret_aux: f64,
    /// Barre basse : réussite auxiliaire moyenne pendant la vie (descriptif).
    pub y_aux_life: f64,
    /// Barre haute : `Y_stat` (256 épisodes `x2 = 0`).
    pub y_stat: f64,
    /// Barre haute : `Ret_stat` (64 sondes `World::retention(t, false)`).
    pub ret_stat: f64,
    /// Descriptifs de la vie stationnaire : dommage, enquête.
    pub h_stat: f64,
    pub q_stat: f64,
    /// Conflits révélés pendant la vie stationnaire (`q = 1, v = 1, z = b`) et protections
    /// (`a = 1 − z`) — la prédiction §4.2 à mesurer, pas un critère.
    pub conflicts_life: u32,
    pub protected_life: u32,
    /// Mêmes comptes sur les 64 sondes stationnaires.
    pub conflicts_probe: u32,
    pub protected_probe: u32,
    /// Sondes stationnaires : dommage moyen.
    pub h_probe: f64,
}

fn delta_reinforce(advantage: f64, decision: bool, f: &Forward) -> f64 {
    -advantage * (bit(decision) - f.p_clamped)
}

/// Barre basse (§5.3).
pub fn lower_bar(genome: &Genome, root: u64, case: Case) -> Result<(f64, f64)> {
    let mut learner = born(genome, root, case.scale)?;
    let mut hits_life = 0u32;
    for t in 0..LIFE {
        let d = Address { root, phase: phase::LOW_LIFE, t: t as u32, ..Address::default() };
        let mut world = World::sampled(d, false, false);
        world.v = true;
        let o1 = world.observation(true);
        let (a, fa) = learner.decide(&o1, Head::Act, d.stream(4).draw(0).uniform())?;
        let target = !world.z;
        let y_aux = a == target;
        hits_life += u32::from(y_aux);
        let reward = 16.0 * bit(y_aux) / 17.0;
        let delta_a = match case.rule {
            Rule::L0 => delta_reinforce(reward - learner.baseline, a, &fa),
            Rule::L1 => fa.p_clamped - bit(target),
        };
        learner.update(None, &fa, delta_a, reward)?;
    }
    let before = learner.organism.weights().to_owned();
    let mut hits = 0u32;
    for t in 0..PROBES {
        let d = Address { root, phase: phase::LOW_PROBES, t: t as u32, ..Address::default() };
        let mut world = World::retention(t, false);
        world.v = true;
        let o1 = world.observation(true);
        let (a, _) = learner.decide(&o1, Head::Act, d.stream(4).draw(0).uniform())?;
        hits += u32::from(a == !world.z);
    }
    debug_assert_eq!(&before, learner.organism.weights());
    Ok((hits as f64 / PROBES as f64, hits_life as f64 / LIFE as f64))
}

/// Barre haute (§5.4), sur un organisme neuf indépendant de la barre basse.
pub fn upper_bar(genome: &Genome, root: u64, case: Case, out: &mut Replicate) -> Result<()> {
    let mut learner = born(genome, root, case.scale)?;
    let (mut y, mut h, mut q_count) = (0u32, 0u32, 0u32);
    for t in 0..LIFE {
        let d = Address { root, phase: phase::HIGH_LIFE, t: t as u32, ..Address::default() };
        let mut world = World::sampled(d, false, false);
        world.x[2] = false;
        let b = world.correct_action();
        let o0 = world.observation(false);
        let (q, fq) = learner.decide(&o0, Head::Investigate, d.stream(3).draw(0).uniform())?;
        let o1 = world.observation(q);
        let (a, fa) = learner.decide(&o1, Head::Act, d.stream(4).draw(0).uniform())?;
        let outcome = world.outcome(q, a);
        y += u32::from(outcome.y);
        h += u32::from(outcome.h);
        q_count += u32::from(q);
        if q && world.v && world.z == b {
            out.conflicts_life += 1;
            out.protected_life += u32::from(a == !world.z);
        }
        let advantage = outcome.reward - learner.baseline;
        let delta_q = delta_reinforce(advantage, q, &fq);
        let delta_a = match case.rule {
            Rule::L0 => delta_reinforce(advantage, a, &fa),
            Rule::L1 => {
                // b reconstruite après l'action depuis (a, y), sans oracle supplémentaire.
                let reconstructed = if outcome.y { a } else { !a };
                debug_assert_eq!(reconstructed, b);
                fa.p_clamped - bit(reconstructed)
            }
        };
        learner.update(Some((&fq, delta_q)), &fa, delta_a, outcome.reward)?;
    }
    out.y_stat = y as f64 / LIFE as f64;
    out.h_stat = h as f64 / LIFE as f64;
    out.q_stat = q_count as f64 / LIFE as f64;
    let before = learner.organism.weights().to_owned();
    let (mut y, mut h) = (0u32, 0u32);
    for t in 0..PROBES {
        let d = Address { root, phase: phase::HIGH_PROBES, t: t as u32, ..Address::default() };
        let world = World::retention(t, false);
        let b = world.correct_action();
        let o0 = world.observation(false);
        let (q, _) = learner.decide(&o0, Head::Investigate, d.stream(3).draw(0).uniform())?;
        let o1 = world.observation(q);
        let (a, _) = learner.decide(&o1, Head::Act, d.stream(4).draw(0).uniform())?;
        let outcome = world.outcome(q, a);
        y += u32::from(outcome.y);
        h += u32::from(outcome.h);
        if q && world.v && world.z == b {
            out.conflicts_probe += 1;
            out.protected_probe += u32::from(a == !world.z);
        }
    }
    debug_assert_eq!(&before, learner.organism.weights());
    out.ret_stat = y as f64 / PROBES as f64;
    out.h_probe = h as f64 / PROBES as f64;
    Ok(())
}

/// Une répétition complète d'une case : barre basse puis barre haute.
pub fn replicate(root: u64, case: Case) -> Result<Replicate> {
    let genome = initial_genome(root);
    let mut out = Replicate { root, ..Replicate::default() };
    let (ret_aux, y_aux_life) = lower_bar(&genome, root, case)?;
    out.ret_aux = ret_aux;
    out.y_aux_life = y_aux_life;
    upper_bar(&genome, root, case, &mut out)?;
    Ok(out)
}

// ---------------------------------------------------------------------------
// Statistique (§5.2) et décision (§6)
// ---------------------------------------------------------------------------

/// Indices bootstrap `Address { root: 0, stream: 19, i: b, t: slot }.integer(16)`,
/// communs à toutes les mesures et à toutes les cases.
pub fn bootstrap_indices() -> Vec<[u8; REPLICATES]> {
    (0..PILOT_BOOTSTRAPS)
        .map(|b| {
            let mut row = [0u8; REPLICATES];
            for (slot, value) in row.iter_mut().enumerate() {
                let d = Address { root: 0, stream: BOOTSTRAP_STREAM, i: b as u32, t: slot as u32, ..Address::default() };
                *value = d.integer(REPLICATES as u64) as u8;
            }
            row
        })
        .collect()
}

/// Borne inférieure unilatérale à 95 % : rang 5 000 des 100 000 moyennes triées.
pub fn low95(values: &[f64; REPLICATES], indices: &[[u8; REPLICATES]]) -> f64 {
    assert_eq!(indices.len(), PILOT_BOOTSTRAPS);
    let mut means: Vec<f64> = indices
        .iter()
        .map(|row| row.iter().map(|&i| values[i as usize]).sum::<f64>() / REPLICATES as f64)
        .collect();
    means.sort_by(f64::total_cmp);
    means[LOW_RANK]
}

pub fn mean(values: &[f64; REPLICATES]) -> f64 {
    values.iter().sum::<f64>() / REPLICATES as f64
}

/// Résumé d'une case sur 16 racines.
#[derive(Clone, Debug)]
pub struct CaseSummary {
    pub case: Case,
    pub replicates: Vec<Replicate>,
    pub mean_ret_aux: f64,
    pub low_ret_aux: f64,
    pub mean_y_stat: f64,
    pub low_y_stat: f64,
    pub mean_ret_stat: f64,
    pub low_ret_stat: f64,
    pub lower_bar: bool,
    pub upper_bar: bool,
}
impl CaseSummary {
    pub fn qualified(&self) -> bool {
        self.lower_bar && self.upper_bar
    }
}

pub fn summarize(case: Case, replicates: Vec<Replicate>, indices: &[[u8; REPLICATES]]) -> Result<CaseSummary> {
    if replicates.len() != REPLICATES {
        return Err(format!("{} répétitions au lieu de 16", replicates.len()));
    }
    let column = |f: fn(&Replicate) -> f64| -> [f64; REPLICATES] { std::array::from_fn(|k| f(&replicates[k])) };
    let ret_aux = column(|r| r.ret_aux);
    let y_stat = column(|r| r.y_stat);
    let ret_stat = column(|r| r.ret_stat);
    let (low_ret_aux, low_y_stat, low_ret_stat) = (low95(&ret_aux, indices), low95(&y_stat, indices), low95(&ret_stat, indices));
    Ok(CaseSummary {
        case,
        mean_ret_aux: mean(&ret_aux),
        mean_y_stat: mean(&y_stat),
        mean_ret_stat: mean(&ret_stat),
        lower_bar: low_ret_aux > THRESHOLD,
        upper_bar: low_y_stat > THRESHOLD && low_ret_stat > THRESHOLD,
        low_ret_aux,
        low_y_stat,
        low_ret_stat,
        replicates,
    })
}

/// Règle de sélection §6.3 : maximum de `Ret_stat` moyen parmi les cases qualifiées,
/// égalités à `10⁻⁶` départagées par l'ordre de priorité préenregistré.
pub fn select(summaries: &[CaseSummary]) -> Option<Case> {
    let best = summaries.iter().filter(|s| s.qualified()).map(|s| s.mean_ret_stat).fold(f64::NEG_INFINITY, f64::max);
    CASES.into_iter().find(|case| {
        summaries.iter().any(|s| s.case == *case && s.qualified() && s.mean_ret_stat >= best - 1e-6)
    })
}

pub fn run_case(roots: std::ops::Range<u64>, case: Case, indices: &[[u8; REPLICATES]]) -> Result<CaseSummary> {
    let replicates = roots.map(|root| replicate(root, case)).collect::<Result<Vec<_>>>()?;
    summarize(case, replicates, indices)
}

// ---------------------------------------------------------------------------
// Étape 0 — témoin statique à poids fixés (§3)
// ---------------------------------------------------------------------------

/// Témoin analytique `w*` (publié en clair) : tous les poids sont nuls sauf
///
/// * cellule 0, couche d'entrée (colonnes `x0 x1 x2 x3 q qv qz biais`) :
///   `h0 = tanh(8x0 + 8x1 − 4)` (OU), `h1 = tanh(6x0 + 6x1 − 8)` (ET),
///   `h2 = tanh(8qv − 4)` (risque révélé), `h3 = tanh(8qv − 8qz − 4)` (risque révélé, `z = 0`) ;
/// * cellule 1 : `h0 = tanh(8)` (unité constante) ;
/// * `W_lat = 0`, donc `h2 = h` ; lecture de la tête `a` répartie à parts égales sur `h` et `h2` :
///   `logit = 4 h0 − 4 h1 − 4 h2 + 12 h3 + 4 · const`.
///
/// Sans risque révélé : `logit ≈ 4(OU − ET) − 4`, soit `≈ ±4` selon `x0 ⊕ x1` ;
/// risque révélé : `logit ≈ 4X + 12 ≥ 12` si `z = 0`, `≈ 4X − 12 ≤ −4` si `z = 1`.
pub fn witness_weights() -> [[f64; WEIGHTS_PER_CELL]; CELLS] {
    let mut w = [[0.0; WEIGHTS_PER_CELL]; CELLS];
    let input = |r: usize, k: usize| layout::W_IN + r * 8 + k;
    // colonnes : 0 x0, 1 x1, 2 x2, 3 x3, 4 q, 5 qv, 6 qz, 7 biais
    w[0][input(0, 0)] = 8.0;
    w[0][input(0, 1)] = 8.0;
    w[0][input(0, 7)] = -4.0;
    w[0][input(1, 0)] = 6.0;
    w[0][input(1, 1)] = 6.0;
    w[0][input(1, 7)] = -8.0;
    w[0][input(2, 5)] = 8.0;
    w[0][input(2, 7)] = -4.0;
    w[0][input(3, 5)] = 8.0;
    w[0][input(3, 6)] = -8.0;
    w[0][input(3, 7)] = -4.0;
    w[1][input(0, 7)] = 8.0;
    for (k, value) in [2.0, -2.0, -2.0, 6.0].into_iter().enumerate() {
        w[0][layout::W_A + k] = value;
        w[0][layout::W_A + HIDDEN + k] = value;
    }
    w[1][layout::W_A] = 2.0;
    w[1][layout::W_A + HIDDEN] = 2.0;
    w
}

/// Organisme du témoin : gains transcriptomiques uniformes `τ = 1`, aucune cellule masquée,
/// poids `w*`. Construit par l'état sérialisé canonique de `genome v1` (sa relecture vérifie
/// les domaines : `τ ∈ [0,1]`, `|w| ≤ 8`), sans modifier la bibliothèque.
pub fn witness_organism() -> Result<Organism> {
    let genome = initial_genome(0xC0FFEE); // structure seulement : τ et w sont remplacés
    let organism = Organism::develop(genome.clone(), &Rng::new(0xC0FFEE), Coord::default(), false);
    let hexes = |values: Vec<f64>| {
        let parts: Vec<String> = values.iter().map(|v| format!("0x{:016x}", v.to_bits())).collect();
        format!("[{}]", parts.join(", "))
    };
    let ones = hexes(vec![1.0; CELLS * genome::GENES]);
    let weights = hexes(witness_weights().iter().flatten().copied().collect());
    let state: String = organism
        .to_state_string()
        .lines()
        .map(|line| {
            if line.starts_with("tau = ") {
                format!("tau = {ones}\n")
            } else if line.starts_with("weights = ") {
                format!("weights = {weights}\n")
            } else {
                format!("{line}\n")
            }
        })
        .collect();
    let witness = Organism::from_state_string(&state, genome).map_err(|e| e.to_string())?;
    if witness.transcriptomes().iter().flatten().any(|&t| t != 1.0) || witness.masked().iter().any(|&m| m) {
        return Err("témoin : τ non uniforme ou cellule masquée".into());
    }
    Ok(witness)
}

/// Une ligne du test statique.
#[derive(Clone, Copy, Debug)]
pub struct StaticRow {
    pub x0: bool,
    pub x1: bool,
    pub v: bool,
    pub z: bool,
    pub target: bool,
    /// `P(a = cible)` avec l'observation du moteur (`qz = q·z`, `World::observation`).
    pub p_engine: f64,
    /// `P(a = cible)` avec l'observation écrite au §3 (`qz = v·z`).
    pub p_protocol: f64,
}

/// Étape 0 : les 16 tuples `(x0, x1, v, z)`, `x2 = x3 = 0`, `q = 1`.
/// Le témoin passe s'il dépasse `0,80` sur les 16 tuples pour les deux écritures de `qz`.
pub fn static_witness() -> Result<(Vec<StaticRow>, bool)> {
    let witness = witness_organism()?;
    let p_target = |o: &[f64; 7], target: bool| {
        let p = witness.forward(&Obs::from_vector(o), Head::Act).p;
        if target { p } else { 1.0 - p }
    };
    let mut rows = Vec::with_capacity(16);
    for n in 0..16 {
        let world = World { x: [n & 8 != 0, n & 4 != 0, false, false], v: n & 2 != 0, z: n & 1 != 0, post_switch: false };
        let target = if world.v { !world.z } else { world.x[0] ^ world.x[1] };
        let engine = world.observation(true);
        let mut protocol = engine;
        protocol[6] = bit(world.v && world.z);
        rows.push(StaticRow {
            x0: world.x[0],
            x1: world.x[1],
            v: world.v,
            z: world.z,
            target,
            p_engine: p_target(&engine, target),
            p_protocol: p_target(&protocol, target),
        });
    }
    let pass = rows.iter().all(|r| r.p_engine > STATIC_THRESHOLD && r.p_protocol > STATIC_THRESHOLD);
    Ok((rows, pass))
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn w0_decoder_is_bit_identical_to_genome_v1() {
        for root in [3u64, 2026, 900] {
            let g = initial_genome(root);
            let ours = decode_weights(&g, 20.0).unwrap();
            let theirs = genome::decode_initial_weights(&g);
            for c in 0..CELLS {
                for k in 0..WEIGHTS_PER_CELL {
                    assert_eq!(ours[c][k].to_bits(), theirs[c][k].to_bits());
                }
            }
            let born0 = born(&g, root, Scale::W0).unwrap();
            assert_eq!(born0.organism.weights(), &theirs);
        }
    }

    #[test]
    fn w1_weights_are_ten_times_wider_and_nothing_else_changes() {
        let g = initial_genome(2026);
        let a = born(&g, 2026, Scale::W0).unwrap();
        let b = born(&g, 2026, Scale::W1).unwrap();
        assert_eq!(a.organism.transcriptomes(), b.organism.transcriptomes());
        assert_eq!(a.organism.plasticity(), b.organism.plasticity());
        let mut max = 0.0f64;
        for c in 0..CELLS {
            for k in 0..WEIGHTS_PER_CELL {
                let (x, y) = (a.organism.weights()[c][k], b.organism.weights()[c][k]);
                assert!((y - 10.0 * x).abs() < 1e-12);
                assert!(y.abs() <= 0.5);
                max = max.max(y.abs());
            }
        }
        assert!(max > 0.05, "W1 doit dépasser l'échelle v1");
    }

    #[test]
    fn pilot_coordinates_match_protocol_table() {
        let g = initial_genome(2026);
        let rng = Rng::new(2026);
        let expected = Genome::random(EncodingKind::Indirect, &rng, Coord { stream: 1, phase: 1, ..Coord::default() });
        assert_eq!(g, expected);
        let v1 = Genome::random(EncodingKind::Indirect, &rng, Coord { stream: 1, ..Coord::default() });
        assert_ne!(g, v1, "phase 1 : génomes distincts des génomes v1 de phase 0");
    }

    /// Sous (W0, L0), l'update du pilote = `Organism::learn_episode` (bit à bit).
    #[test]
    fn l0_update_matches_learn_episode_bitwise() {
        let root = 2026;
        let g = initial_genome(root);
        let mut ours = born(&g, root, Scale::W0).unwrap();
        let mut engine = ours.organism.clone();
        for t in 0..LIFE {
            let d = Address { root, phase: 77, t: t as u32, ..Address::default() };
            let mut world = World::sampled(d, false, false);
            world.x[2] = false;
            let o0 = world.observation(false);
            let (q, fq) = ours.decide(&o0, Head::Investigate, d.stream(3).draw(0).uniform()).unwrap();
            let o1 = world.observation(q);
            let (a, fa) = ours.decide(&o1, Head::Act, d.stream(4).draw(0).uniform()).unwrap();
            let outcome = world.outcome(q, a);
            let adv = outcome.reward - ours.baseline;
            ours.update(Some((&fq, delta_reinforce(adv, q, &fq))), &fa, delta_reinforce(adv, a, &fa), outcome.reward).unwrap();
            let efq = engine.forward(&Obs::from_vector(&o0), Head::Investigate);
            let efa = engine.forward(&Obs::from_vector(&o1), Head::Act);
            engine.learn_episode(&efq, q as u8, &efa, a as u8, outcome.reward);
            assert_eq!(ours.baseline.to_bits(), engine.baseline().to_bits());
            assert_eq!(ours.organism.weights(), engine.weights(), "t = {t}");
            for c in 0..CELLS {
                assert_eq!(ours.utility[c].to_bits(), engine.utilities()[c].to_bits());
            }
        }
    }

    #[test]
    fn l1_delta_is_cross_entropy_on_reconstructed_target() {
        // b = a si y, 1 − a sinon ; δ_a = p̃_a − b ; la tête q garde REINFORCE.
        let g = initial_genome(2026);
        let learner = born(&g, 2026, Scale::W0).unwrap();
        let world = World { x: [true, false, false, false], v: true, z: true, post_switch: false };
        let o1 = world.observation(true);
        let fa = learner.organism.forward(&Obs::from_vector(&o1), Head::Act);
        for a in [false, true] {
            let y = world.outcome(true, a).y;
            let b = if y { a } else { !a };
            assert_eq!(b, world.correct_action());
            let delta = fa.p_clamped - bit(b);
            assert!(if b { delta < 0.0 } else { delta > 0.0 });
        }
    }

    #[test]
    fn lower_bar_never_trains_the_q_head() {
        let root = 2026;
        let g = initial_genome(root);
        let mut learner = born(&g, root, Scale::W1).unwrap();
        let start = *learner.organism.weights();
        let world = World { x: [true, false, false, false], v: true, z: false, post_switch: false };
        let o1 = world.observation(true);
        let fa = learner.organism.forward(&Obs::from_vector(&o1), Head::Act);
        learner.update(None, &fa, fa.p_clamped - 1.0, 16.0 / 17.0).unwrap();
        for c in 0..CELLS {
            for k in layout::W_Q..layout::W_A {
                assert_eq!(start[c][k], learner.organism.weights()[c][k], "W_q ne doit pas bouger");
            }
        }
        assert_ne!(&start, learner.organism.weights());
    }

    #[test]
    fn probes_do_not_learn_and_runs_are_deterministic() {
        let case = CASES[3];
        let a = replicate(905, case).unwrap();
        let b = replicate(905, case).unwrap();
        assert_eq!(a, b);
        for v in [a.ret_aux, a.y_stat, a.ret_stat, a.h_stat, a.q_stat] {
            assert!((0.0..=1.0).contains(&v));
        }
    }

    #[test]
    fn static_witness_passes_on_all_sixteen_tuples() {
        let (rows, pass) = static_witness().unwrap();
        assert_eq!(rows.len(), 16);
        for r in &rows {
            assert!(r.p_engine > 0.95 && r.p_protocol > 0.95, "{r:?}");
        }
        assert!(pass);
    }

    #[test]
    fn bootstrap_low_bound_and_selection_rule() {
        let indices: Vec<[u8; REPLICATES]> = (0..PILOT_BOOTSTRAPS)
            .map(|b| std::array::from_fn(|s| ((b * 7 + s * 3) % 16) as u8))
            .collect();
        let constant = [0.6; REPLICATES];
        assert!((low95(&constant, &indices) - 0.6).abs() < 1e-12);
        let fake = |case: Case, ret: f64, ok: bool| CaseSummary {
            case,
            replicates: Vec::new(),
            mean_ret_aux: 0.0,
            low_ret_aux: 0.0,
            mean_y_stat: 0.0,
            low_y_stat: 0.0,
            mean_ret_stat: ret,
            low_ret_stat: 0.0,
            lower_bar: ok,
            upper_bar: ok,
        };
        let s = vec![fake(CASES[0], 0.7, false), fake(CASES[1], 0.6, true), fake(CASES[2], 0.6 + 5e-7, true), fake(CASES[3], 0.65, true)];
        assert_eq!(select(&s), Some(CASES[3]));
        let s = vec![fake(CASES[0], 0.7, false), fake(CASES[1], 0.65, true), fake(CASES[2], 0.65 + 5e-7, true), fake(CASES[3], 0.6, true)];
        assert_eq!(select(&s), Some(CASES[1]), "égalité à 1e-6 : priorité préenregistrée");
        let s = vec![fake(CASES[0], 0.7, false)];
        assert_eq!(select(&s), None);
    }
}
