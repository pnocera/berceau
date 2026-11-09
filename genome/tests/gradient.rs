//! Vérification numérique des gradients (différences finies) et contrats d'apprentissage.

use genome::{Coord, EncodingKind, Genome, Head, Obs, Organism, Rng, CELLS, WEIGHTS_PER_CELL};

fn organism(root: u64, kind: EncodingKind, uniform: bool) -> Organism {
    let rng = Rng::new(root);
    let g = Genome::random(kind, &rng, Coord::default().with_g(1));
    Organism::develop(g, &rng, Coord::default().with_g(1), uniform)
}

/// L(w) = −adv [log π(q|o0) + log π(a|o1)] pour une référence m fixée à zéro.
fn loss(o: &Organism, o0: &Obs, o1: &Obs, q: u8, a: u8, reward: f64) -> f64 {
    let fq = o.forward(o0, Head::Investigate);
    let fa = o.forward(o1, Head::Act);
    o.episode_loss(&fq, q, &fa, a, reward)
}

#[test]
fn analytic_gradients_match_finite_differences() {
    for (root, kind, uniform) in [(1u64, EncodingKind::Indirect, false), (2, EncodingKind::Direct, false), (3, EncodingKind::Indirect, true)] {
        let mut o = organism(root, kind, uniform);
        // agrandir les poids pour que les non-linéarités soient actives, en restant
        // loin de la borne ±8 (sinon la perturbation est écrêtée et la différence finie vaut 0)
        let w0 = *o.weights();
        for c in 0..CELLS {
            for k in 0..WEIGHTS_PER_CELL {
                let v = w0[c][k] * 8.0 + 0.02 * (((c * 7 + k * 13) % 11) as f64 - 5.0);
                o.set_weight(c, k, v);
            }
        }
        let o0 = Obs::initial([1, 0, 1, 0]);
        let o1 = Obs::final_obs([1, 0, 1, 0], 1, 1, 0);
        let (q, a, reward) = (1u8, 0u8, 1.0);
        let fq = o.forward(&o0, Head::Investigate);
        let fa = o.forward(&o1, Head::Act);
        // hors de la zone bornée [2^-16, 1 - 2^-16], où la perte écrêtée est plate
        for f in [&fq, &fa] {
            assert!(f.p > 1e-3 && f.p < 1.0 - 1e-3, "probabilité saturée {} (root {root})", f.p);
            assert_eq!(f.p, f.p_clamped);
        }
        let grads = o.episode_gradients(&fq, q, &fa, a, reward - o.baseline()).weights;

        let eps = 1e-6;
        let mut checked = 0;
        let mut max_rel = 0.0f64;
        for c in [0usize, 3, 9, 15] {
            for k in 0..WEIGHTS_PER_CELL {
                let base = o.weights()[c][k];
                let mut plus = o.clone();
                plus.set_weight(c, k, base + eps);
                let mut minus = o.clone();
                minus.set_weight(c, k, base - eps);
                let num = (loss(&plus, &o0, &o1, q, a, reward) - loss(&minus, &o0, &o1, q, a, reward)) / (2.0 * eps);
                let ana = grads[c][k];
                let scale = num.abs().max(ana.abs()).max(1e-3);
                let rel = (num - ana).abs() / scale;
                max_rel = max_rel.max(rel);
                assert!(rel < 1e-5, "cellule {c} poids {k} : numérique {num} analytique {ana} (root {root})");
                checked += 1;
            }
        }
        assert_eq!(checked, 4 * WEIGHTS_PER_CELL);
        assert!(max_rel < 1e-5);
    }
}

#[test]
fn culture_gradient_matches_finite_differences() {
    let mut o = organism(5, EncodingKind::Indirect, false);
    let w0 = *o.weights();
    for c in 0..CELLS {
        for k in 0..WEIGHTS_PER_CELL {
            o.set_weight(c, k, w0[c][k] * 30.0);
        }
    }
    let rec = genome::Record::from_experience([0, 1, 1, 0], 1, 1, 0, 1, 0, 0);
    let before = o.culture_loss(&rec);
    // une présentation : w' = w − (1/64) ∇L ; pour un pas petit la perte doit baisser
    let mut after = o.clone();
    after.learn_record(&rec);
    assert!(after.culture_loss(&rec) < before);
    // le pas réalisé est exactement −(1/64)·∇, vérifié sur un poids par différences finies
    let (c, k) = (4usize, 50usize);
    let eps = 1e-6;
    let mut plus = o.clone();
    plus.set_weight(c, k, o.weights()[c][k] + eps);
    let mut minus = o.clone();
    minus.set_weight(c, k, o.weights()[c][k] - eps);
    let num = (plus.culture_loss(&rec) - minus.culture_loss(&rec)) / (2.0 * eps);
    let step = after.weights()[c][k] - o.weights()[c][k];
    assert!((step + num / 64.0).abs() < 1e-7, "pas {step} attendu {}", -num / 64.0);
}

#[test]
fn learning_moves_toward_rewarded_choices() {
    let mut o = organism(8, EncodingKind::Indirect, false);
    let o0 = Obs::initial([1, 1, 0, 0]);
    let o1 = Obs::final_obs([1, 1, 0, 0], 0, 0, 0);
    let p_before = o.probability(&o1, Head::Act);
    // récompenser systématiquement a=1 avec q=0 ; référence m part de zéro
    for _ in 0..400 {
        let fq = o.forward(&o0, Head::Investigate);
        let fa = o.forward(&o1, Head::Act);
        o.learn_episode(&fq, 0, &fa, 1, 1.0);
    }
    let p_after = o.probability(&o1, Head::Act);
    assert!(p_after > p_before, "avant {p_before} après {p_after}");
    assert_eq!(o.age(), 400);
}
