use cantrip_api::*;
use cantrip_mechanics::tokens::{self, LANDER};
use cantrip_mechanics::triggers::THIS_DIES;
use cantrip_oracle as oracle;

pub static CARD: CardDef = CardDef {
    abilities: &[triggered(THIS_DIES, tokens::create_a_lander, "When this creature dies, create a Lander token.")],
    ..oracle::BEAMSAW_PROSPECTOR
};

pub static EXTRA: &[&CardDef] = &[&LANDER];
