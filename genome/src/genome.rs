//! Génome développemental (PROTOCOLE.md §3.1, §5.1 ; SPEC_GENOME.md §2).
//!
//! Un génome est une suite de mots de 16 bits : un bloc réglementaire commun de
//! 64 mots, puis un bloc d'encodage de 128 mots (indirect, 3 072 bits au total)
//! ou de 1 024 mots (direct, 17 408 bits au total). Il est immuable : aucune
//! méthode ne modifie un génome existant ; le croisement et la mutation produisent
//! de nouveaux génomes.

use crate::rng::{stream, Coord, Rng};
use crate::sha256::sha256_hex;

pub const REGULATORY_WORDS: usize = 64;
pub const INDIRECT_WORDS: usize = 128;
pub const DIRECT_WORDS: usize = 1024;
pub const FORMAT_VERSION: u32 = 1;

/// Nature du bloc d'encodage.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EncodingKind {
    /// 128 mots : décodeur indirect (SPEC_GENOME.md §3.2).
    Indirect,
    /// 1 024 mots : un mot par poids initial, `(2k/65535 - 1)/20`.
    Direct,
}

impl EncodingKind {
    pub const fn words(self) -> usize {
        match self {
            EncodingKind::Indirect => INDIRECT_WORDS,
            EncodingKind::Direct => DIRECT_WORDS,
        }
    }
    pub const fn name(self) -> &'static str {
        match self {
            EncodingKind::Indirect => "indirect",
            EncodingKind::Direct => "direct",
        }
    }
    pub fn parse(s: &str) -> Option<EncodingKind> {
        match s {
            "indirect" => Some(EncodingKind::Indirect),
            "direct" => Some(EncodingKind::Direct),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GenomeError {
    /// Texte canonique mal formé ; le message dit où.
    Format(String),
    /// Nombre de mots incohérent avec la nature annoncée.
    WordCount { expected: usize, found: usize },
    /// Croisement entre deux natures d'encodage différentes.
    KindMismatch,
    /// Octets d'un fichier scellé dont l'empreinte ne correspond pas.
    Fingerprint { expected: String, found: String },
}

impl std::fmt::Display for GenomeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GenomeError::Format(m) => write!(f, "format de génome invalide : {m}"),
            GenomeError::WordCount { expected, found } => {
                write!(f, "nombre de mots d'encodage invalide : attendu {expected}, trouvé {found}")
            }
            GenomeError::KindMismatch => write!(f, "croisement impossible entre encodages de natures différentes"),
            GenomeError::Fingerprint { expected, found } => {
                write!(f, "empreinte de génome invalide : attendu {expected}, trouvé {found}")
            }
        }
    }
}

impl std::error::Error for GenomeError {}

/// Génome immuable.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Genome {
    kind: EncodingKind,
    regulatory: [u16; REGULATORY_WORDS],
    encoding: Vec<u16>,
}

impl Genome {
    /// Construit un génome à partir de ses mots ; vérifie la taille du bloc d'encodage.
    pub fn from_words(kind: EncodingKind, regulatory: [u16; REGULATORY_WORDS], encoding: Vec<u16>) -> Result<Genome, GenomeError> {
        if encoding.len() != kind.words() {
            return Err(GenomeError::WordCount { expected: kind.words(), found: encoding.len() });
        }
        Ok(Genome { kind, regulatory, encoding })
    }

    /// Génome aléatoire : flux `GENETIC_INIT`, `draw = indice du mot` (réglementaire
    /// puis encodage), chaque mot étant les 16 bits de poids faible d'un `u64` frais.
    /// `base` fournit `g, i, j, phase, t` ; `stream` et `draw` sont imposés ici.
    pub fn random(kind: EncodingKind, rng: &Rng, base: Coord) -> Genome {
        let mut regulatory = [0u16; REGULATORY_WORDS];
        for (w, slot) in regulatory.iter_mut().enumerate() {
            *slot = rng.draw(base.with_stream(stream::GENETIC_INIT).with_draw(w as u32)).next_u16();
        }
        let mut encoding = Vec::with_capacity(kind.words());
        for w in 0..kind.words() {
            let idx = (REGULATORY_WORDS + w) as u32;
            encoding.push(rng.draw(base.with_stream(stream::GENETIC_INIT).with_draw(idx)).next_u16());
        }
        Genome { kind, regulatory, encoding }
    }

    pub fn kind(&self) -> EncodingKind {
        self.kind
    }
    pub fn regulatory(&self) -> &[u16; REGULATORY_WORDS] {
        &self.regulatory
    }
    pub fn encoding(&self) -> &[u16] {
        &self.encoding
    }
    /// Nombre total de mots `L` (64 + 128 ou 64 + 1 024).
    pub fn word_count(&self) -> usize {
        REGULATORY_WORDS + self.encoding.len()
    }
    /// Taille génétique en bits : 3 072 (indirect) ou 17 408 (direct).
    pub fn bits(&self) -> usize {
        16 * self.word_count()
    }
    /// Mot d'indice `w` dans l'ordre réglementaire puis encodage.
    pub fn word(&self, w: usize) -> u16 {
        if w < REGULATORY_WORDS {
            self.regulatory[w]
        } else {
            self.encoding[w - REGULATORY_WORDS]
        }
    }
    /// Tous les mots, réglementaires puis encodage.
    pub fn words(&self) -> impl Iterator<Item = u16> + '_ {
        self.regulatory.iter().copied().chain(self.encoding.iter().copied())
    }

    fn from_word_iter(kind: EncodingKind, mut it: impl Iterator<Item = u16>) -> Genome {
        let mut regulatory = [0u16; REGULATORY_WORDS];
        for slot in regulatory.iter_mut() {
            *slot = it.next().expect("mots réglementaires");
        }
        let encoding: Vec<u16> = it.collect();
        debug_assert_eq!(encoding.len(), kind.words());
        Genome { kind, regulatory, encoding }
    }

    /// Croisement (PROTOCOLE §5.1) : pour chaque mot aligné `w`, Bernoulli(1/2) au
    /// flux `CROSSOVER`, `draw = w` ; vrai → mot de `a`, faux → mot de `b`.
    /// Ne mute pas : appeler ensuite `mutated`.
    pub fn crossover(a: &Genome, b: &Genome, rng: &Rng, base: Coord) -> Result<Genome, GenomeError> {
        if a.kind != b.kind {
            return Err(GenomeError::KindMismatch);
        }
        let words = (0..a.word_count()).map(|w| {
            let from_a = rng.draw(base.with_stream(stream::CROSSOVER).with_draw(w as u32)).bernoulli(0.5);
            if from_a { a.word(w) } else { b.word(w) }
        });
        Ok(Genome::from_word_iter(a.kind, words))
    }

    /// Mutation (PROTOCOLE §5.1) : pour chacun des `L` mots, au flux `MUTATION`,
    /// `draw = w` : `U < 1/L` → remplacement par un entier uniforme sur `0..=65535`
    /// (tiré ensuite dans le même tirage, sans biais de modulo). Une mutation attendue
    /// par descendant quelle que soit la taille de l'encodage.
    pub fn mutated(&self, rng: &Rng, base: Coord) -> Genome {
        let l = self.word_count();
        let p = 1.0 / l as f64;
        let words = (0..l).map(|w| {
            let mut d = rng.draw(base.with_stream(stream::MUTATION).with_draw(w as u32));
            if d.bernoulli(p) {
                d.uniform_usize(65536) as u16
            } else {
                self.word(w)
            }
        });
        Genome::from_word_iter(self.kind, words)
    }

    /// Texte canonique (déterministe, un seul rendu possible par génome) :
    /// ```text
    /// berceau-genome-format = 1
    /// kind = "indirect"
    /// regulatory = [w0, w1, ..., w63]
    /// encoding = [e0, e1, ...]
    /// ```
    /// Nombres en décimal, séparés par `", "`, fin de ligne `\n`.
    pub fn to_canonical_string(&self) -> String {
        fn list(words: &[u16]) -> String {
            let parts: Vec<String> = words.iter().map(|w| w.to_string()).collect();
            format!("[{}]", parts.join(", "))
        }
        format!(
            "berceau-genome-format = {}\nkind = \"{}\"\nregulatory = {}\nencoding = {}\n",
            FORMAT_VERSION,
            self.kind.name(),
            list(&self.regulatory),
            list(&self.encoding),
        )
    }

    /// Lecture du texte canonique. Tolère les espaces superflus et les lignes vides,
    /// mais exige les quatre clés, la version 1 et des tailles exactes.
    pub fn parse(text: &str) -> Result<Genome, GenomeError> {
        let mut version: Option<u32> = None;
        let mut kind: Option<EncodingKind> = None;
        let mut regulatory: Option<Vec<u16>> = None;
        let mut encoding: Option<Vec<u16>> = None;

        for (n, raw) in text.lines().enumerate() {
            let line = raw.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let (key, value) = line
                .split_once('=')
                .ok_or_else(|| GenomeError::Format(format!("ligne {} sans '='", n + 1)))?;
            let key = key.trim();
            let value = value.trim();
            match key {
                "berceau-genome-format" => {
                    version = Some(value.parse().map_err(|_| GenomeError::Format(format!("version illisible : {value}")))?)
                }
                "kind" => {
                    let name = value.trim_matches('"');
                    kind = Some(EncodingKind::parse(name).ok_or_else(|| GenomeError::Format(format!("nature inconnue : {name}")))?)
                }
                "regulatory" => regulatory = Some(parse_list(value)?),
                "encoding" => encoding = Some(parse_list(value)?),
                other => return Err(GenomeError::Format(format!("clé inconnue : {other}"))),
            }
        }

        match version {
            Some(FORMAT_VERSION) => {}
            Some(v) => return Err(GenomeError::Format(format!("version non prise en charge : {v}"))),
            None => return Err(GenomeError::Format("version absente".into())),
        }
        let kind = kind.ok_or_else(|| GenomeError::Format("kind absent".into()))?;
        let regulatory = regulatory.ok_or_else(|| GenomeError::Format("regulatory absent".into()))?;
        let encoding = encoding.ok_or_else(|| GenomeError::Format("encoding absent".into()))?;
        if regulatory.len() != REGULATORY_WORDS {
            return Err(GenomeError::Format(format!("{} mots réglementaires au lieu de {}", regulatory.len(), REGULATORY_WORDS)));
        }
        let mut reg = [0u16; REGULATORY_WORDS];
        reg.copy_from_slice(&regulatory);
        Genome::from_words(kind, reg, encoding)
    }

    /// Empreinte SHA-256 (hexadécimal) du texte canonique.
    pub fn fingerprint(&self) -> String {
        sha256_hex(self.to_canonical_string().as_bytes())
    }

    /// Relecture scellée d'un fichier `genome.toml` : les octets doivent avoir exactement
    /// l'empreinte attendue **et** être le rendu canonique du génome qu'ils décrivent
    /// (toute modification, même d'un espace, est refusée).
    pub fn from_sealed_bytes(bytes: &[u8], expected_fingerprint: &str) -> Result<Genome, GenomeError> {
        let found = sha256_hex(bytes);
        if found != expected_fingerprint {
            return Err(GenomeError::Fingerprint { expected: expected_fingerprint.to_string(), found });
        }
        let text = std::str::from_utf8(bytes).map_err(|_| GenomeError::Format("UTF-8 invalide".into()))?;
        let genome = Genome::parse(text)?;
        if genome.to_canonical_string().as_bytes() != bytes {
            return Err(GenomeError::Format("le fichier n'est pas le rendu canonique du génome".into()));
        }
        Ok(genome)
    }
}

fn parse_list(value: &str) -> Result<Vec<u16>, GenomeError> {
    let inner = value
        .strip_prefix('[')
        .and_then(|v| v.strip_suffix(']'))
        .ok_or_else(|| GenomeError::Format("liste sans crochets".into()))?;
    let inner = inner.trim();
    if inner.is_empty() {
        return Ok(Vec::new());
    }
    inner
        .split(',')
        .map(|s| s.trim().parse::<u16>().map_err(|_| GenomeError::Format(format!("mot illisible : {s:?}"))))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base() -> Coord {
        Coord::default().with_g(1).with_i(2)
    }

    #[test]
    fn sizes() {
        let rng = Rng::new(0);
        let gi = Genome::random(EncodingKind::Indirect, &rng, base());
        let gd = Genome::random(EncodingKind::Direct, &rng, base());
        assert_eq!(gi.word_count(), 192);
        assert_eq!(gi.bits(), 3072);
        assert_eq!(gd.word_count(), 1088);
        assert_eq!(gd.bits(), 17408);
        // même base, même bloc réglementaire : les 64 premiers tirages sont partagés
        assert_eq!(gi.regulatory(), gd.regulatory());
    }

    #[test]
    fn canonical_roundtrip_and_fingerprint() {
        let rng = Rng::new(5);
        for kind in [EncodingKind::Indirect, EncodingKind::Direct] {
            let g = Genome::random(kind, &rng, base());
            let text = g.to_canonical_string();
            let back = Genome::parse(&text).unwrap();
            assert_eq!(g, back);
            assert_eq!(g.fingerprint(), back.fingerprint());
            assert_eq!(g.fingerprint().len(), 64);
            // insensible aux espaces
            let spaced = text.replace(", ", " ,  ");
            assert_eq!(Genome::parse(&spaced).unwrap(), g);
        }
    }

    #[test]
    fn parse_rejects_bad_input() {
        assert!(matches!(Genome::parse("kind = \"indirect\"\n"), Err(GenomeError::Format(_))));
        let rng = Rng::new(5);
        let g = Genome::random(EncodingKind::Indirect, &rng, base());
        let text = g.to_canonical_string().replace("\"indirect\"", "\"direct\"");
        assert!(matches!(Genome::parse(&text), Err(GenomeError::WordCount { .. })));
        let text = g.to_canonical_string().replace("format = 1", "format = 2");
        assert!(matches!(Genome::parse(&text), Err(GenomeError::Format(_))));
    }

    #[test]
    fn crossover_takes_aligned_words() {
        let rng = Rng::new(9);
        let a = Genome::random(EncodingKind::Indirect, &rng, base());
        let b = Genome::random(EncodingKind::Indirect, &rng, base().with_i(7));
        let c = Genome::crossover(&a, &b, &rng, base()).unwrap();
        let mut from_a = 0;
        let mut from_b = 0;
        for w in 0..c.word_count() {
            let cw = c.word(w);
            assert!(cw == a.word(w) || cw == b.word(w));
            if cw == a.word(w) { from_a += 1 } else { from_b += 1 }
        }
        assert!(from_a > 40 && from_b > 40, "a={from_a} b={from_b}");
        let d = Genome::random(EncodingKind::Direct, &rng, base());
        assert_eq!(Genome::crossover(&a, &d, &rng, base()), Err(GenomeError::KindMismatch));
    }

    #[test]
    fn mutation_rate_is_one_word_expected() {
        let rng = Rng::new(21);
        let g = Genome::random(EncodingKind::Direct, &rng, base());
        let mut changed_total = 0;
        let trials = 300;
        for k in 0..trials {
            let m = g.mutated(&rng, base().with_t(k));
            changed_total += (0..g.word_count()).filter(|&w| m.word(w) != g.word(w)).count();
        }
        let mean = changed_total as f64 / trials as f64;
        // attendu ≈ 1 - (1/65536) ≈ 1 ; tolérance large
        assert!(mean > 0.7 && mean < 1.35, "mutations moyennes {mean}");
    }

    #[test]
    fn sealed_bytes_detect_any_change() {
        let rng = Rng::new(13);
        let g = Genome::random(EncodingKind::Indirect, &rng, base());
        let bytes = g.to_canonical_string().into_bytes();
        let fp = g.fingerprint();
        assert_eq!(Genome::from_sealed_bytes(&bytes, &fp).unwrap(), g);
        let mut spaced = bytes.clone();
        spaced.push(b'\n');
        assert!(matches!(Genome::from_sealed_bytes(&spaced, &fp), Err(GenomeError::Fingerprint { .. })));
        // même un fichier équivalent mais non canonique est refusé si on en donne l'empreinte
        let fp2 = sha256_hex(&spaced);
        assert!(matches!(Genome::from_sealed_bytes(&spaced, &fp2), Err(GenomeError::Format(_))));
    }

    #[test]
    fn mutation_is_deterministic_per_coordinates() {
        let rng = Rng::new(21);
        let g = Genome::random(EncodingKind::Indirect, &rng, base());
        assert_eq!(g.mutated(&rng, base()), g.mutated(&rng, base()));
        assert_eq!(g.fingerprint(), g.clone().fingerprint());
    }
}
