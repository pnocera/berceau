//! Développement : 32 tours de régulation synchrone sur 16 cellules en anneau
//! (PROTOCOLE.md §3.1, §3.3 ; SPEC_GENOME.md §4).
//!
//! Chaque cellule porte un état réglementaire `s ∈ [−8, 8]^4` dont le transcriptome
//! est `τ = σ(s) ∈ (0,1)^4` (domaine convexe). Les seules asymétries initiales sont la
//! position normalisée `p = c/16` et un bruit local tiré du flux `DEVELOPMENT`.

use crate::decoder::word_to_unit;
use crate::genome::Genome;
use crate::rng::{stream, Coord, Rng};
use crate::CELLS;

/// Nombre de gènes régulés par cellule (dimension du transcriptome).
pub const GENES: usize = 4;
/// Tours de développement.
pub const DEV_ROUNDS: u32 = 32;
/// Pas d'intégration du réseau de régulation.
pub const DEV_STEP: f64 = 0.25;
/// Borne de l'état réglementaire.
pub const STATE_CLAMP: f64 = 8.0;
/// Taux de plasticité minimal et maximal : `η = 2^(−10 + 5 τ₃) ∈ [1/1024, 1/32]`.
pub const ETA_MIN: f64 = 1.0 / 1024.0;
pub const ETA_MAX: f64 = 1.0 / 32.0;

/// Rôle des quatre gènes du transcriptome.
pub mod gene {
    /// Gain de lecture pour la tête `q` (enquête).
    pub const GATE_Q: usize = 0;
    /// Gain de lecture pour la tête `a` (action).
    pub const GATE_A: usize = 1;
    /// Couplage latéral dans l'anneau.
    pub const LATERAL: usize = 2;
    /// Plasticité locale.
    pub const PLASTICITY: usize = 3;
}

/// Paramètres réglementaires décodés des 64 mots communs.
///
/// Ordre des mots : `a` (16), `b` (16), `c` (4), `pos` (12), `bias` (4),
/// `sigma` (4), `init` (4), `decay` (4). Échelles : `2u` pour a, b, c, pos, bias, init ;
/// `0,5 |u|` pour sigma ; `0,25 (u + 1)` pour decay.
#[derive(Clone, Debug, PartialEq)]
pub struct RegParams {
    pub a: [[f64; GENES]; GENES],
    pub b: [[f64; GENES]; GENES],
    pub c: [f64; GENES],
    pub pos: [[f64; 3]; GENES],
    pub bias: [f64; GENES],
    pub sigma: [f64; GENES],
    pub init: [f64; GENES],
    pub decay: [f64; GENES],
}

impl RegParams {
    pub fn from_genome(genome: &Genome) -> RegParams {
        let r = genome.regulatory();
        let mut idx = 0usize;
        let mut next = || {
            let v = word_to_unit(r[idx]);
            idx += 1;
            v
        };
        let mut a = [[0.0; GENES]; GENES];
        for row in a.iter_mut() {
            for v in row.iter_mut() {
                *v = 2.0 * next();
            }
        }
        let mut b = [[0.0; GENES]; GENES];
        for row in b.iter_mut() {
            for v in row.iter_mut() {
                *v = 2.0 * next();
            }
        }
        let mut c = [0.0; GENES];
        for v in c.iter_mut() {
            *v = 2.0 * next();
        }
        let mut pos = [[0.0; 3]; GENES];
        for row in pos.iter_mut() {
            for v in row.iter_mut() {
                *v = 2.0 * next();
            }
        }
        let mut bias = [0.0; GENES];
        for v in bias.iter_mut() {
            *v = 2.0 * next();
        }
        let mut sigma = [0.0; GENES];
        for v in sigma.iter_mut() {
            *v = 0.5 * next().abs();
        }
        let mut init = [0.0; GENES];
        for v in init.iter_mut() {
            *v = 2.0 * next();
        }
        let mut decay = [0.0; GENES];
        for v in decay.iter_mut() {
            *v = 0.25 * (next() + 1.0);
        }
        debug_assert_eq!(idx, 64);
        RegParams { a, b, c, pos, bias, sigma, init, decay }
    }
}

/// Résultat du développement : états réglementaires gelés pour la vie.
#[derive(Clone, Debug, PartialEq)]
pub struct Regulation {
    pub state: [[f64; GENES]; CELLS],
    pub tau: [[f64; GENES]; CELLS],
    pub eta: [f64; CELLS],
}

pub fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

fn logit(p: f64) -> f64 {
    let p = p.clamp(1e-9, 1.0 - 1e-9);
    (p / (1.0 - p)).ln()
}

/// `η = 2^(−10 + 5 τ₃)`.
pub fn plasticity_from_gene(tau_plasticity: f64) -> f64 {
    2f64.powf(-10.0 + 5.0 * tau_plasticity)
}

/// Caractéristiques de position de la cellule `c` : `(p, cos 2πp, sin 2πp)`, `p = c/16`.
pub fn position_features(c: usize) -> [f64; 3] {
    let p = c as f64 / CELLS as f64;
    let ang = 2.0 * std::f64::consts::PI * p;
    [p, ang.cos(), ang.sin()]
}

/// Développe un organisme : 32 tours synchrones.
///
/// Bruit : pour le tour `r` et la cellule `c`, tirage au flux `DEVELOPMENT`,
/// `t = r`, `draw = c` ; quatre uniformes `U` donnent `ξ = 2U − 1`.
/// `base` fournit `g, i, j, phase` ; `stream`, `t` et `draw` sont imposés ici.
///
/// Mode témoin `uniform` (PROTOCOLE §3.3) : après chaque tour, chaque transcriptome
/// est remplacé par la moyenne des 16 et chaque taux de plasticité par leur moyenne ;
/// les états sont ramenés à `logit(τ̄)`. Rien d'autre n'est synchronisé.
pub fn develop(params: &RegParams, rng: &Rng, base: Coord, uniform: bool) -> Regulation {
    let mut state = [params.init; CELLS];
    let mut tau = [[0.0; GENES]; CELLS];
    let mut eta = [0.0; CELLS];
    for c in 0..CELLS {
        for k in 0..GENES {
            tau[c][k] = sigmoid(state[c][k]);
        }
        eta[c] = plasticity_from_gene(tau[c][gene::PLASTICITY]);
    }

    for round in 0..DEV_ROUNDS {
        // mises à jour candidates, calculées de façon synchrone sur l'état courant
        let mut new_state = state;
        for c in 0..CELLS {
            let left = tau[(c + CELLS - 1) % CELLS];
            let right = tau[(c + 1) % CELLS];
            let feats = position_features(c);
            let mut d = rng.draw(base.with_stream(stream::DEVELOPMENT).with_t(round).with_draw(c as u32));
            for k in 0..GENES {
                let mut drive = params.bias[k];
                for l in 0..GENES {
                    drive += params.a[k][l] * tau[c][l];
                    drive += params.b[k][l] * 0.5 * (left[l] + right[l]);
                }
                drive += params.c[k] * (right[k] - left[k]);
                for f in 0..3 {
                    drive += params.pos[k][f] * feats[f];
                }
                let xi = 2.0 * d.uniform_f64() - 1.0;
                drive += params.sigma[k] * xi;
                let s = (1.0 - params.decay[k]) * state[c][k] + DEV_STEP * drive;
                new_state[c][k] = s.clamp(-STATE_CLAMP, STATE_CLAMP);
            }
        }
        state = new_state;
        for c in 0..CELLS {
            for k in 0..GENES {
                tau[c][k] = sigmoid(state[c][k]);
            }
            eta[c] = plasticity_from_gene(tau[c][gene::PLASTICITY]);
        }
        if uniform {
            let mut mean_tau = [0.0; GENES];
            let mut mean_eta = 0.0;
            for c in 0..CELLS {
                for k in 0..GENES {
                    mean_tau[k] += tau[c][k] / CELLS as f64;
                }
                mean_eta += eta[c] / CELLS as f64;
            }
            for c in 0..CELLS {
                tau[c] = mean_tau;
                eta[c] = mean_eta;
                for k in 0..GENES {
                    state[c][k] = logit(mean_tau[k]);
                }
            }
        }
    }
    Regulation { state, tau, eta }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::genome::EncodingKind;

    #[test]
    fn params_consume_all_words_with_declared_scales() {
        let rng = Rng::new(1);
        let g = Genome::random(EncodingKind::Indirect, &rng, Coord::default());
        let p = RegParams::from_genome(&g);
        for row in p.a.iter().chain(p.b.iter()) {
            for v in row {
                assert!(v.abs() <= 2.0);
            }
        }
        for v in p.sigma {
            assert!((0.0..=0.5).contains(&v));
        }
        for v in p.decay {
            assert!((0.0..=0.5).contains(&v));
        }
        let r = g.regulatory();
        assert_eq!(p.a[0][0], 2.0 * word_to_unit(r[0]));
        assert_eq!(p.decay[3], 0.25 * (word_to_unit(r[63]) + 1.0));
    }

    #[test]
    fn regulation_in_domain() {
        let rng = Rng::new(8);
        for root in 0..6u64 {
            let rng2 = Rng::new(root);
            let g = Genome::random(EncodingKind::Indirect, &rng2, Coord::default());
            let p = RegParams::from_genome(&g);
            let reg = develop(&p, &rng, Coord::default(), false);
            for c in 0..CELLS {
                for k in 0..GENES {
                    assert!(reg.tau[c][k] > 0.0 && reg.tau[c][k] < 1.0);
                    assert!(reg.state[c][k].abs() <= STATE_CLAMP);
                }
                assert!(reg.eta[c] >= ETA_MIN - 1e-15 && reg.eta[c] <= ETA_MAX + 1e-15, "{}", reg.eta[c]);
            }
        }
    }

    #[test]
    fn uniform_mode_has_zero_variance_and_shares_updates() {
        let rng = Rng::new(8);
        let g = Genome::random(EncodingKind::Indirect, &rng, Coord::default());
        let p = RegParams::from_genome(&g);
        let u = develop(&p, &rng, Coord::default(), true);
        for c in 1..CELLS {
            assert_eq!(u.tau[c], u.tau[0]);
            assert_eq!(u.eta[c], u.eta[0]);
        }
        let free = develop(&p, &rng, Coord::default(), false);
        let var: f64 = (0..GENES)
            .map(|k| {
                let m = free.tau.iter().map(|t| t[k]).sum::<f64>() / CELLS as f64;
                free.tau.iter().map(|t| (t[k] - m).powi(2)).sum::<f64>() / CELLS as f64
            })
            .sum();
        assert!(var > 0.0, "le mode libre doit pouvoir se différencier");
    }

    #[test]
    fn deterministic_by_coordinates() {
        let rng = Rng::new(3);
        let g = Genome::random(EncodingKind::Direct, &rng, Coord::default());
        let p = RegParams::from_genome(&g);
        let base = Coord::default().with_g(2).with_i(5).with_j(1).with_phase(0);
        assert_eq!(develop(&p, &rng, base, false), develop(&p, &rng, base, false));
        assert_ne!(develop(&p, &rng, base, false), develop(&p, &rng, base.with_j(2), false));
    }
}
