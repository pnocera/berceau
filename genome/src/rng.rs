//! Flot aléatoire adressé (PROTOCOLE.md §5.2).
//!
//! Chaque tirage est identifié par une coordonnée `(stream, g, i, j, phase, t, draw)`
//! et une racine `u64`. Le bloc `counter` du tirage vaut
//! `SHA-256("BERCEAU-V1" ‖ 0x00 ‖ root:u64 LE ‖ stream,g,i,j,phase,t,draw,counter : u32 LE)`.
//! Les octets du condensat sont consommés dans l'ordre ; `counter` part de zéro et
//! augmente pour obtenir des blocs supplémentaires du même tirage. Il n'existe aucun
//! compteur mutable global : deux consommateurs ne peuvent se décaler l'un l'autre.

use crate::sha256::sha256;

/// Préfixe de domaine, octet zéro compris.
pub const DOMAIN: &[u8; 11] = b"BERCEAU-V1\0";

/// Codes de `stream` (PROTOCOLE.md §5.2).
pub mod stream {
    pub const ENVIRONMENT: u32 = 0;
    pub const GENETIC_INIT: u32 = 1;
    pub const DEVELOPMENT: u32 = 2;
    pub const POLICY_Q: u32 = 3;
    pub const POLICY_A: u32 = 4;
    pub const PARENT_SELECTION: u32 = 5;
    pub const CROSSOVER: u32 = 6;
    pub const MUTATION: u32 = 7;
    pub const RENEWAL: u32 = 8;
    pub const BOOTSTRAP: u32 = 9;
    pub const EXPLORATORY: u32 = 10;
}

/// Codes de `phase` (PROTOCOLE.md §5.2). L'évaluation finale ajoute `FINAL_OFFSET`.
pub mod phase {
    pub const DEV_DONOR: u32 = 0;
    pub const CULTURE_DONOR: u32 = 1;
    pub const LIFE_DONOR: u32 = 2;
    pub const DEV_STUDENT: u32 = 3;
    pub const CULTURE_STUDENT: u32 = 4;
    pub const LIFE_STUDENT: u32 = 5;
    pub const PROBES: u32 = 6;
    pub const EXPORT: u32 = 7;
    pub const FINAL_OFFSET: u32 = 16;
}

/// Coordonnée complète d'un tirage.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Coord {
    pub stream: u32,
    pub g: u32,
    pub i: u32,
    pub j: u32,
    pub phase: u32,
    pub t: u32,
    pub draw: u32,
}

impl Coord {
    /// Coordonnée dont tous les champs valent zéro sauf `stream`.
    pub const fn of_stream(stream: u32) -> Coord {
        Coord { stream, g: 0, i: 0, j: 0, phase: 0, t: 0, draw: 0 }
    }
    pub const fn with_stream(mut self, stream: u32) -> Coord { self.stream = stream; self }
    pub const fn with_g(mut self, g: u32) -> Coord { self.g = g; self }
    pub const fn with_i(mut self, i: u32) -> Coord { self.i = i; self }
    pub const fn with_j(mut self, j: u32) -> Coord { self.j = j; self }
    pub const fn with_phase(mut self, phase: u32) -> Coord { self.phase = phase; self }
    pub const fn with_t(mut self, t: u32) -> Coord { self.t = t; self }
    pub const fn with_draw(mut self, draw: u32) -> Coord { self.draw = draw; self }

    /// Message haché pour le bloc `counter` de ce tirage sous la racine `root`.
    pub fn message(&self, root: u64, counter: u32) -> [u8; 11 + 8 + 8 * 4] {
        let mut m = [0u8; 11 + 8 + 32];
        m[..11].copy_from_slice(DOMAIN);
        m[11..19].copy_from_slice(&root.to_le_bytes());
        let fields = [self.stream, self.g, self.i, self.j, self.phase, self.t, self.draw, counter];
        for (k, f) in fields.iter().enumerate() {
            m[19 + 4 * k..23 + 4 * k].copy_from_slice(&f.to_le_bytes());
        }
        m
    }
}

/// Générateur adressé : une racine, aucun état mutable.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rng {
    root: u64,
}

impl Rng {
    pub const fn new(root: u64) -> Rng {
        Rng { root }
    }

    pub const fn root(&self) -> u64 {
        self.root
    }

    /// Bloc `counter` (32 octets) du tirage `coord`.
    pub fn block(&self, coord: Coord, counter: u32) -> [u8; 32] {
        sha256(&coord.message(self.root, counter))
    }

    /// Ouvre un tirage : un consommateur séquentiel des octets de `coord`.
    pub fn draw(&self, coord: Coord) -> Draw {
        Draw { root: self.root, coord, counter: 0, buf: self.block(coord, 0), pos: 0 }
    }
}

/// Consommateur séquentiel des octets d'un tirage adressé.
#[derive(Clone, Debug)]
pub struct Draw {
    root: u64,
    coord: Coord,
    counter: u32,
    buf: [u8; 32],
    pos: usize,
}

impl Draw {
    pub fn coord(&self) -> Coord {
        self.coord
    }

    fn next_byte(&mut self) -> u8 {
        if self.pos == 32 {
            self.counter += 1;
            self.buf = sha256(&self.coord.message(self.root, self.counter));
            self.pos = 0;
        }
        let b = self.buf[self.pos];
        self.pos += 1;
        b
    }

    /// Prochain mot de 64 bits, lu en little-endian dans le flot d'octets.
    pub fn next_u64(&mut self) -> u64 {
        let mut bytes = [0u8; 8];
        for b in bytes.iter_mut() {
            *b = self.next_byte();
        }
        u64::from_le_bytes(bytes)
    }

    /// Mot de 16 bits : les 16 bits de poids faible d'un `u64` frais.
    pub fn next_u16(&mut self) -> u16 {
        (self.next_u64() & 0xffff) as u16
    }

    /// Uniforme réel dans `[0,1)` : 53 bits de poids fort d'un `u64`, divisés par 2^53.
    pub fn uniform_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / 9_007_199_254_740_992.0
    }

    /// Bernoulli(p) : vaut 1 si `U < p`.
    pub fn bernoulli(&mut self, p: f64) -> bool {
        self.uniform_f64() < p
    }

    /// Entier uniforme sur `0..n`, sans biais de modulo : lire des `u64` jusqu'à
    /// `k < floor(2^64 / n) * n`, puis rendre `k mod n`.
    pub fn uniform_usize(&mut self, n: usize) -> usize {
        assert!(n > 0, "uniform_usize: n doit être strictement positif");
        let n128 = n as u128;
        let limit: u128 = ((1u128 << 64) / n128) * n128;
        loop {
            let k = self.next_u64();
            if (k as u128) < limit {
                return (k % n as u64) as usize;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn message_layout() {
        let c = Coord { stream: 1, g: 2, i: 3, j: 4, phase: 5, t: 6, draw: 7 };
        let m = c.message(0x0102030405060708, 9);
        assert_eq!(&m[..11], b"BERCEAU-V1\0");
        assert_eq!(&m[11..19], &[8, 7, 6, 5, 4, 3, 2, 1]);
        assert_eq!(&m[19..23], &[1, 0, 0, 0]);
        assert_eq!(&m[47..51], &[9, 0, 0, 0]);
        assert_eq!(m.len(), 51);
    }

    #[test]
    fn deterministic_and_addressed() {
        let rng = Rng::new(0);
        let a = rng.draw(Coord::of_stream(0)).next_u64();
        let b = rng.draw(Coord::of_stream(0)).next_u64();
        let c = rng.draw(Coord::of_stream(0).with_draw(1)).next_u64();
        let d = Rng::new(1).draw(Coord::of_stream(0)).next_u64();
        assert_eq!(a, b);
        assert_ne!(a, c);
        assert_ne!(a, d);
    }

    #[test]
    fn blocks_chain_by_counter() {
        let rng = Rng::new(3);
        let coord = Coord::of_stream(2).with_t(5);
        let mut d = rng.draw(coord);
        let mut first = Vec::new();
        for _ in 0..8 {
            first.push(d.next_u64());
        }
        // les 4 premiers u64 viennent du bloc 0, les 4 suivants du bloc 1
        let b0 = rng.block(coord, 0);
        let b1 = rng.block(coord, 1);
        let u = |b: &[u8; 32], k: usize| u64::from_le_bytes(b[8 * k..8 * k + 8].try_into().unwrap());
        assert_eq!(first[0], u(&b0, 0));
        assert_eq!(first[3], u(&b0, 3));
        assert_eq!(first[4], u(&b1, 0));
        assert_eq!(first[7], u(&b1, 3));
    }

    #[test]
    fn uniform_ranges() {
        let rng = Rng::new(11);
        let mut d = rng.draw(Coord::of_stream(10));
        for _ in 0..2000 {
            let u = d.uniform_f64();
            assert!((0.0..1.0).contains(&u));
        }
        for n in [1usize, 2, 3, 7, 16, 30, 65536] {
            for _ in 0..200 {
                assert!(d.uniform_usize(n) < n);
            }
        }
        assert_eq!(d.uniform_usize(1), 0);
    }

    #[test]
    fn uniform_f64_is_top_53_bits() {
        let rng = Rng::new(0);
        let coord = Coord::of_stream(0);
        let k = rng.draw(coord).next_u64();
        let u = rng.draw(coord).uniform_f64();
        assert_eq!(u, (k >> 11) as f64 / 2f64.powi(53));
    }

    #[test]
    fn bernoulli_frequency() {
        let rng = Rng::new(7);
        let mut d = rng.draw(Coord::of_stream(10).with_draw(1));
        let n = 20_000;
        let hits = (0..n).filter(|_| d.bernoulli(0.25)).count() as f64 / n as f64;
        assert!((hits - 0.25).abs() < 0.02, "fréquence {hits}");
    }
}
