//! Décodeur des poids initiaux (SPEC_GENOME.md §3).
//!
//! * mot → unité : `u(k) = 2k/65535 − 1 ∈ [−1, 1]` ;
//! * encodage direct : `w[c][k] = u(e[64c + k]) / 20` ;
//! * encodage indirect : pour chaque indice de poids `k` (0..64), deux mots
//!   `f0 = u(e[2k])`, `f1 = u(e[2k+1])`, et pour chaque cellule `c` (0..16)
//!   `w[c][k] = tanh(2 (f0 + f1 cos(2πc/16 + πk/32))) / 20`.
//!
//! Les deux décodeurs produisent 1 024 poids dans `[−1/20, 1/20]`.

use crate::genome::{EncodingKind, Genome, REGULATORY_WORDS};
use crate::{CELLS, WEIGHTS_PER_CELL};

/// Diviseur des poids initiaux : `w = u / 20`.
pub const INITIAL_WEIGHT_DIVISOR: f64 = 20.0;
/// Amplitude maximale d'un poids initial.
pub const INITIAL_WEIGHT_BOUND: f64 = 1.0 / INITIAL_WEIGHT_DIVISOR;

/// `u(k) = 2k/65535 − 1`.
pub fn word_to_unit(k: u16) -> f64 {
    2.0 * (k as f64) / 65535.0 - 1.0
}

/// Quantification 16 bits d'un poids de `[−1/20, 1/20]` : `k = round((20w + 1) · 65535 / 2)`,
/// borné dans `0..=65535`. Sert à initialiser le témoin direct de C1 (PROTOCOLE §5.1).
pub fn quantize_weight(w: f64) -> u16 {
    let x = (20.0 * w + 1.0) * 65535.0 / 2.0;
    x.round().clamp(0.0, 65535.0) as u16
}

/// Poids initiaux `[cellule][indice]` décodés du génome.
pub fn decode_initial_weights(genome: &Genome) -> [[f64; WEIGHTS_PER_CELL]; CELLS] {
    let e = genome.encoding();
    let mut w = [[0.0; WEIGHTS_PER_CELL]; CELLS];
    match genome.kind() {
        EncodingKind::Direct => {
            for c in 0..CELLS {
                for k in 0..WEIGHTS_PER_CELL {
                    w[c][k] = word_to_unit(e[c * WEIGHTS_PER_CELL + k]) / INITIAL_WEIGHT_DIVISOR;
                }
            }
        }
        EncodingKind::Indirect => {
            for k in 0..WEIGHTS_PER_CELL {
                let f0 = word_to_unit(e[2 * k]);
                let f1 = word_to_unit(e[2 * k + 1]);
                let phi = std::f64::consts::PI * (k as f64) / 32.0;
                for c in 0..CELLS {
                    let theta = 2.0 * std::f64::consts::PI * (c as f64) / (CELLS as f64);
                    w[c][k] = (2.0 * (f0 + f1 * (theta + phi).cos())).tanh() / INITIAL_WEIGHT_DIVISOR;
                }
            }
        }
    }
    w
}

/// Génome direct dont les mots quantifient les poids donnés (bloc réglementaire copié).
pub fn direct_genome_from_weights(regulatory_source: &Genome, weights: &[[f64; WEIGHTS_PER_CELL]; CELLS]) -> Genome {
    let mut enc = Vec::with_capacity(EncodingKind::Direct.words());
    for cell in weights.iter() {
        for &w in cell.iter() {
            enc.push(quantize_weight(w));
        }
    }
    let mut reg = [0u16; REGULATORY_WORDS];
    reg.copy_from_slice(regulatory_source.regulatory());
    Genome::from_words(EncodingKind::Direct, reg, enc).expect("1024 mots")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rng::{Coord, Rng};

    #[test]
    fn unit_and_quantization() {
        assert_eq!(word_to_unit(0), -1.0);
        assert_eq!(word_to_unit(65535), 1.0);
        assert!((word_to_unit(32768) - 1.0 / 65535.0).abs() < 1e-12);
        assert_eq!(quantize_weight(-0.05), 0);
        assert_eq!(quantize_weight(0.05), 65535);
        assert_eq!(quantize_weight(-1.0), 0);
        assert_eq!(quantize_weight(1.0), 65535);
        // aller-retour : l'erreur de quantification est bornée par un demi-pas
        for k in [0u16, 1, 1000, 32767, 32768, 65534, 65535] {
            let w = word_to_unit(k) / 20.0;
            assert_eq!(quantize_weight(w), k);
        }
    }

    #[test]
    fn decoders_stay_in_range() {
        let rng = Rng::new(2);
        for kind in [EncodingKind::Indirect, EncodingKind::Direct] {
            let g = Genome::random(kind, &rng, Coord::default());
            let w = decode_initial_weights(&g);
            for cell in w.iter() {
                for &x in cell.iter() {
                    assert!(x.abs() <= INITIAL_WEIGHT_BOUND + 1e-15, "{x}");
                }
            }
        }
    }

    #[test]
    fn direct_formula() {
        let rng = Rng::new(2);
        let g = Genome::random(EncodingKind::Direct, &rng, Coord::default());
        let w = decode_initial_weights(&g);
        let e = g.encoding();
        assert_eq!(w[3][17], (2.0 * e[3 * 64 + 17] as f64 / 65535.0 - 1.0) / 20.0);
    }

    #[test]
    fn indirect_varies_across_cells_and_quantizes_closely() {
        let rng = Rng::new(4);
        let g = Genome::random(EncodingKind::Indirect, &rng, Coord::default());
        let w = decode_initial_weights(&g);
        let distinct = (0..16).map(|c| w[c][5].to_bits()).collect::<std::collections::HashSet<_>>().len();
        assert!(distinct > 4, "le motif en anneau doit varier selon la cellule");
        let d = direct_genome_from_weights(&g, &w);
        assert_eq!(d.kind(), EncodingKind::Direct);
        assert_eq!(d.regulatory(), g.regulatory());
        let wd = decode_initial_weights(&d);
        for c in 0..16 {
            for k in 0..64 {
                assert!((wd[c][k] - w[c][k]).abs() <= 0.5 * (0.1 / 65535.0) + 1e-15);
            }
        }
    }
}
