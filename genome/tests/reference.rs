//! Vecteurs de référence gelés (SPEC_GENOME.md §9). Toute divergence signale un
//! changement du contrat numérique et impose une nouvelle version.

use genome::{sha256_hex, Coord, EncodingKind, Genome, Head, Obs, Organism, Rng};

fn hex(bytes: &[u8]) -> String {
    genome::to_hex(bytes)
}

#[test]
fn print_reference_vectors() {
    // Ce test imprime les vecteurs ; `frozen_reference_vectors` les fige.
    let rng = Rng::new(0);
    let c0 = Coord::of_stream(0);
    println!("block(root=0, all zero, counter=0) = {}", hex(&rng.block(c0, 0)));
    println!("first u64 = {}", rng.draw(c0).next_u64());
    println!("uniform_f64 = {:.17}", rng.draw(c0).uniform_f64());
    let c1 = Coord { stream: 7, g: 3, i: 11, j: 2, phase: 5, t: 128, draw: 4 };
    println!("block(root=29, 7/3/11/2/5/128/4, counter=1) = {}", hex(&Rng::new(29).block(c1, 1)));
    let g = Genome::random(EncodingKind::Indirect, &rng, Coord::default());
    println!("genome indirect root0 fingerprint = {}", g.fingerprint());
    println!("genome indirect root0 words[0..4] = {:?}", &g.words().take(4).collect::<Vec<_>>());
    let gd = Genome::random(EncodingKind::Direct, &rng, Coord::default());
    println!("genome direct root0 fingerprint = {}", gd.fingerprint());
    let o = Organism::develop(g.clone(), &rng, Coord::default(), false);
    println!("tau[0] = {:?}", o.transcriptomes()[0]);
    println!("eta[0] = {:.17}", o.plasticity()[0]);
    println!("w[0][0] = {:.17}", o.weights()[0][0]);
    let p = o.probability(&Obs::initial([1, 0, 1, 0]), Head::Investigate);
    println!("p(q=1 | 1010) = {:.17}", p);
    let pa = o.probability(&Obs::final_obs([1, 0, 1, 0], 1, 1, 0), Head::Act);
    println!("p(a=1 | 1010,1,1,0) = {:.17}", pa);
    println!("state fingerprint at birth = {}", o.state_fingerprint());
    println!("sha256(canonical) = {}", sha256_hex(g.to_canonical_string().as_bytes()));
}

fn close(a: f64, b: f64) -> bool {
    (a - b).abs() <= 1e-15 * a.abs().max(b.abs()).max(1.0)
}

#[test]
fn frozen_reference_vectors() {
    let rng = Rng::new(0);
    let c0 = Coord::of_stream(0);
    assert_eq!(hex(&rng.block(c0, 0)), "04f51dc3403e4d9d05f0579b2a92b7ff904fc6760fa21b88be239f5472c3c400");
    assert_eq!(rng.draw(c0).next_u64(), 11334784285033755908);
    assert!(close(rng.draw(c0).uniform_f64(), 0.61445988732440759));
    let c1 = Coord { stream: 7, g: 3, i: 11, j: 2, phase: 5, t: 128, draw: 4 };
    assert_eq!(hex(&Rng::new(29).block(c1, 1)), "641a20d4b873e6e218610b6a2605d03da811e61812a0c37b3dffa9d27a08036a");

    let g = Genome::random(EncodingKind::Indirect, &rng, Coord::default());
    assert_eq!(g.fingerprint(), "ed8dd5574bcc5494cbdb58a3d554d36fd0f69c7c986d5430b4de410e3fb32f6c");
    assert_eq!(g.words().take(4).collect::<Vec<_>>(), vec![17187, 6099, 59829, 64961]);
    let gd = Genome::random(EncodingKind::Direct, &rng, Coord::default());
    assert_eq!(gd.fingerprint(), "a533c7eca720a4851d75f01223378fc1fa9c2c581c5b13fdef8ad6a541ec22a6");

    let o = Organism::develop(g, &rng, Coord::default(), false);
    let tau0 = o.transcriptomes()[0];
    let expected = [0.22458518189065363, 0.2613963326542555, 0.9185649831505319, 0.7037908610354033];
    for k in 0..4 {
        assert!(close(tau0[k], expected[k]), "tau[0][{k}] = {}", tau0[k]);
    }
    assert!(close(o.plasticity()[0], 0.01119465831685055));
    assert!(close(o.weights()[0][0], -0.01179755708718819));
    assert!(close(o.probability(&Obs::initial([1, 0, 1, 0]), Head::Investigate), 0.49484177123013706));
    assert!(close(o.probability(&Obs::final_obs([1, 0, 1, 0], 1, 1, 0), Head::Act), 0.50043193605695158));
    assert_eq!(o.state_fingerprint(), "f4fc6163f9c56676013e80ac5b32405c5c420399fa3ff565537d875295f44db3");
}
