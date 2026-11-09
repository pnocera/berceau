//! `genome` — bibliothèque commune du laboratoire et du berceau.
//!
//! Elle implémente, en Rust pur et sans dépendance, les contrats de
//! `PROTOCOLE.md` (v1) et les équations de `SPEC_GENOME.md` :
//!
//! * [`sha256`] : SHA-256 FIPS 180-4, empreintes ;
//! * [`rng`] : flot aléatoire adressé `BERCEAU-V1` (coordonnées `stream,g,i,j,phase,t,draw`) ;
//! * [`genome`] : génome immuable (64 mots réglementaires + 128 ou 1 024 mots d'encodage),
//!   texte canonique, empreinte, croisement, mutation ;
//! * [`decoder`] : poids initiaux `[−1/20, 1/20]`, quantification pour le témoin direct ;
//! * [`develop`] : 32 tours de régulation sur 16 cellules en anneau, transcriptomes,
//!   taux de plasticité `[1/1024, 1/32]`, témoin uniforme U ;
//! * [`packet`] : paquet culturel de 512 bits, 32 enregistrements ;
//! * [`organism`] : organisme développé, politiques `q`/`a`, gradient d'épisode et culturel,
//!   transferts, renouvellement, masquage.
//!
//! Le génome d'un organisme est scellé à la naissance : aucune fonction publique ne
//! permet de le modifier ; `Organism::verify_fingerprint` recontrôle l'empreinte.

pub mod decoder;
pub mod develop;
pub mod genome;
pub mod organism;
pub mod packet;
pub mod rng;
pub mod sha256;

/// Nombre de cellules de l'anneau.
pub const CELLS: usize = 16;
/// Poids appris par cellule.
pub const WEIGHTS_PER_CELL: usize = 64;
/// Unités cachées par cellule.
pub const HIDDEN: usize = 4;
/// Dimension d'une observation `(x0,x1,x2,x3,q,qv,qz)`.
pub const OBS_DIM: usize = 7;
/// Dimension de l'entrée du réseau : observation + biais.
pub const INPUT_DIM: usize = OBS_DIM + 1;
/// Nombre total de poids appris.
pub const TOTAL_WEIGHTS: usize = CELLS * WEIGHTS_PER_CELL;
/// Version du contrat de la bibliothèque (à geler avant les essais confirmatoires).
pub const GENOME_LIB_VERSION: &str = "genome-v1";

pub use decoder::{
    decode_initial_weights, direct_genome_from_weights, quantize_weight, word_to_unit, INITIAL_WEIGHT_BOUND, INITIAL_WEIGHT_DIVISOR,
};
pub use develop::{
    develop, gene, plasticity_from_gene, position_features, RegParams, Regulation, DEV_ROUNDS, DEV_STEP, ETA_MAX, ETA_MIN, GENES,
    STATE_CLAMP,
};
pub use genome::{EncodingKind, Genome, GenomeError, DIRECT_WORDS, INDIRECT_WORDS, REGULATORY_WORDS};
pub use organism::{
    layout, ActGrads, EpisodeGrads, EpisodeUpdate, Forward, Grads, Head, Obs, Organism, StateError, BASELINE_KEEP, CULTURE_PASSES,
    CULTURE_RATE, PARTIAL_TRANSFER_CELLS, PROB_EPS, STATE_FORMAT_VERSION, UTILITY_KEEP, WEIGHT_CLAMP,
};
pub use packet::{Packet, PacketError, Record, PACKET_BITS, PACKET_BYTES, RECORDS};
pub use rng::{phase, stream, Coord, Draw, Rng, DOMAIN};
pub use sha256::{sha256, sha256_hex, to_hex};

#[cfg(test)]
mod tests {
    #[test]
    fn constants_match_protocol() {
        assert_eq!(super::TOTAL_WEIGHTS, 1024);
        assert_eq!(super::INPUT_DIM, 8);
        assert_eq!(super::CELLS * super::WEIGHTS_PER_CELL * 64, 65_536);
    }
}
