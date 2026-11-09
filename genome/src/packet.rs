//! Paquet culturel : exactement 512 bits (PROTOCOLE.md §4.1).
//!
//! 64 octets = 32 enregistrements de 2 octets, chaque mot lu en little-endian.
//! Bits : 0–3 `x0..x3`, 4 `q`, 5 `v_obs`, 6 `z_obs`, 7 `a`, 8 `y`, 9 `h_obs`, 10–15 réservés (zéro).

use crate::organism::Obs;
use crate::sha256::sha256_hex;

pub const RECORDS: usize = 32;
pub const PACKET_BYTES: usize = 64;
pub const PACKET_BITS: usize = 512;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PacketError {
    WrongLength(usize),
    /// Bits 10–15 non nuls dans l'enregistrement `index`.
    ReservedBits { index: usize, word: u16 },
    /// `v_obs`, `z_obs` ou `h_obs` non nuls alors que `q = 0`.
    HiddenLeak { index: usize, word: u16 },
    /// Avec `q = 1`, `h_obs` doit valoir `v_obs · 1[a = z_obs]` (PROTOCOLE §2.2).
    InconsistentHarm { index: usize, word: u16 },
}

impl std::fmt::Display for PacketError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PacketError::WrongLength(n) => write!(f, "paquet de {n} octets au lieu de {PACKET_BYTES}"),
            PacketError::ReservedBits { index, word } => {
                write!(f, "bits réservés non nuls dans l'enregistrement {index} (0x{word:04x})")
            }
            PacketError::HiddenLeak { index, word } => {
                write!(f, "champs cachés copiés sans enquête dans l'enregistrement {index} (0x{word:04x})")
            }
            PacketError::InconsistentHarm { index, word } => {
                write!(f, "dommage observé incohérent avec v, z, a dans l'enregistrement {index} (0x{word:04x})")
            }
        }
    }
}

impl std::error::Error for PacketError {}

/// Un enregistrement pédagogique.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct Record {
    pub x: [u8; 4],
    pub q: u8,
    pub v_obs: u8,
    pub z_obs: u8,
    pub a: u8,
    pub y: u8,
    pub h_obs: u8,
}

impl Record {
    /// Construit un enregistrement à partir de ce qu'a vécu le professeur, en masquant
    /// `v, z, h` par zéro si `q = 0` (règle anti-fuite du §4.1).
    pub fn from_experience(x: [u8; 4], q: u8, v: u8, z: u8, a: u8, y: u8, h: u8) -> Record {
        let (v_obs, z_obs, h_obs) = if q == 1 { (v & 1, z & 1, h & 1) } else { (0, 0, 0) };
        Record { x: [x[0] & 1, x[1] & 1, x[2] & 1, x[3] & 1], q: q & 1, v_obs, z_obs, a: a & 1, y: y & 1, h_obs }
    }

    pub fn to_word(&self) -> u16 {
        (self.x[0] as u16 & 1)
            | ((self.x[1] as u16 & 1) << 1)
            | ((self.x[2] as u16 & 1) << 2)
            | ((self.x[3] as u16 & 1) << 3)
            | ((self.q as u16 & 1) << 4)
            | ((self.v_obs as u16 & 1) << 5)
            | ((self.z_obs as u16 & 1) << 6)
            | ((self.a as u16 & 1) << 7)
            | ((self.y as u16 & 1) << 8)
            | ((self.h_obs as u16 & 1) << 9)
    }

    pub fn from_word(word: u16, index: usize) -> Result<Record, PacketError> {
        if word & 0xfc00 != 0 {
            return Err(PacketError::ReservedBits { index, word });
        }
        let bit = |n: u16| ((word >> n) & 1) as u8;
        let r = Record {
            x: [bit(0), bit(1), bit(2), bit(3)],
            q: bit(4),
            v_obs: bit(5),
            z_obs: bit(6),
            a: bit(7),
            y: bit(8),
            h_obs: bit(9),
        };
        if r.q == 0 && (r.v_obs | r.z_obs | r.h_obs) != 0 {
            return Err(PacketError::HiddenLeak { index, word });
        }
        if r.q == 1 && r.h_obs != (r.v_obs & (r.a == r.z_obs) as u8) {
            return Err(PacketError::InconsistentHarm { index, word });
        }
        Ok(r)
    }

    /// Vrai si l'enregistrement respecte les deux règles de cohérence du §2.2 et §4.1
    /// (aucune fuite sans enquête ; `h_obs = v_obs · 1[a = z_obs]` avec enquête).
    pub fn is_consistent(&self) -> bool {
        Record::from_word(self.to_word(), 0).is_ok()
    }

    /// Observation initiale démontrée : `o0 = (x, 0, 0, 0)`.
    pub fn obs0(&self) -> Obs {
        Obs::initial(self.x)
    }

    /// Observation finale démontrée : `o1 = (x, q, v_obs, z_obs)`.
    pub fn obs1(&self) -> Obs {
        Obs { x: self.x, q: self.q, qv: self.v_obs, qz: self.z_obs }
    }
}

/// Paquet de 32 enregistrements, 64 octets.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Packet {
    records: [Record; RECORDS],
}

impl Packet {
    pub fn from_records(records: [Record; RECORDS]) -> Packet {
        Packet { records }
    }

    /// Lecture stricte : 64 octets, bits réservés nuls, aucune fuite sans enquête.
    pub fn from_bytes(bytes: &[u8]) -> Result<Packet, PacketError> {
        if bytes.len() != PACKET_BYTES {
            return Err(PacketError::WrongLength(bytes.len()));
        }
        let mut records = [Record::default(); RECORDS];
        for (i, rec) in records.iter_mut().enumerate() {
            let word = u16::from_le_bytes([bytes[2 * i], bytes[2 * i + 1]]);
            *rec = Record::from_word(word, i)?;
        }
        Ok(Packet { records })
    }

    pub fn to_bytes(&self) -> [u8; PACKET_BYTES] {
        let mut out = [0u8; PACKET_BYTES];
        for (i, rec) in self.records.iter().enumerate() {
            out[2 * i..2 * i + 2].copy_from_slice(&rec.to_word().to_le_bytes());
        }
        out
    }

    pub fn records(&self) -> &[Record; RECORDS] {
        &self.records
    }

    /// Empreinte SHA-256 hexadécimale des 64 octets.
    pub fn fingerprint(&self) -> String {
        sha256_hex(&self.to_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normative_examples() {
        // premier exemple du monde, sans enquête : x=(0,0,1,0), q=0, a=0, y=1, h=1 (caché)
        let r = Record::from_experience([0, 0, 1, 0], 0, 1, 0, 0, 1, 1);
        assert_eq!(r.to_word(), 0x0104);
        assert_eq!(r.to_word().to_le_bytes(), [0x04, 0x01]);
        // enquête puis action protectrice : q=1, v=1, z=0, a=1, y=0, h=0
        let r = Record::from_experience([0, 0, 1, 0], 1, 1, 0, 1, 0, 0);
        assert_eq!(r.to_word(), 0x00b4);
        assert_eq!(r.to_word().to_le_bytes(), [0xb4, 0x00]);
        assert_eq!(Record::from_word(0x00b4, 0).unwrap(), r);
    }

    #[test]
    fn hidden_fields_never_copied_without_inquiry() {
        let r = Record::from_experience([1, 1, 1, 1], 0, 1, 1, 1, 1, 1);
        assert_eq!((r.v_obs, r.z_obs, r.h_obs), (0, 0, 0));
        assert_eq!(Record::from_word(0x0020, 3), Err(PacketError::HiddenLeak { index: 3, word: 0x0020 }));
    }

    #[test]
    fn harm_must_match_observed_risk_when_inquired() {
        // q=1, v=1, z=0, a=0 → h doit être 1 : 0x0134 ; avec h=0 (0x0034) refusé
        assert!(Record::from_word(0x0334, 0).is_ok());
        assert_eq!(Record::from_word(0x0134, 2), Err(PacketError::InconsistentHarm { index: 2, word: 0x0134 }));
        // q=1, v=0 : h doit être 0
        assert_eq!(Record::from_word(0x0210, 4), Err(PacketError::InconsistentHarm { index: 4, word: 0x0210 }));
        assert!(Record::from_word(0x0010, 4).is_ok());
        // énumération exhaustive : la cohérence est exactement la règle énoncée
        for word in 0..=u16::MAX {
            let ok = Record::from_word(word, 0).is_ok();
            let bit = |n: u16| ((word >> n) & 1) as u8;
            let (q, v, z, a, h) = (bit(4), bit(5), bit(6), bit(7), bit(9));
            let expected = word & 0xfc00 == 0
                && if q == 0 { v | z | h == 0 } else { h == (v & (a == z) as u8) };
            assert_eq!(ok, expected, "mot 0x{word:04x}");
            if ok {
                assert_eq!(Record::from_word(word, 0).unwrap().to_word(), word);
            }
        }
    }

    #[test]
    fn reserved_bits_and_length_are_errors() {
        assert_eq!(Record::from_word(0x0400, 1), Err(PacketError::ReservedBits { index: 1, word: 0x0400 }));
        assert_eq!(Packet::from_bytes(&[0u8; 63]), Err(PacketError::WrongLength(63)));
        let mut bytes = [0u8; 64];
        bytes[11] = 0x80; // bit 15 de l'enregistrement 5
        assert_eq!(Packet::from_bytes(&bytes), Err(PacketError::ReservedBits { index: 5, word: 0x8000 }));
    }

    #[test]
    fn roundtrip_and_size() {
        let mut recs = [Record::default(); RECORDS];
        for (i, r) in recs.iter_mut().enumerate() {
            let b = |n: usize| ((i >> n) & 1) as u8;
            let (v, z, a) = (b(3), b(4), b(0) ^ b(1));
            *r = Record::from_experience([b(0), b(1), b(2), 0], 1, v, z, a, 1, v & (a == z) as u8);
        }
        let p = Packet::from_records(recs);
        let bytes = p.to_bytes();
        assert_eq!(bytes.len() * 8, PACKET_BITS);
        assert_eq!(Packet::from_bytes(&bytes).unwrap(), p);
        assert_eq!(p.fingerprint().len(), 64);
    }

    #[test]
    fn observations_from_record() {
        let r = Record::from_experience([1, 0, 1, 0], 1, 1, 0, 1, 0, 0);
        assert_eq!(r.obs0(), Obs { x: [1, 0, 1, 0], q: 0, qv: 0, qz: 0 });
        assert_eq!(r.obs1(), Obs { x: [1, 0, 1, 0], q: 1, qv: 1, qz: 0 });
    }
}
