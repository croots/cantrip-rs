use cantrip_api::*;
use cantrip_mechanics::tokens::{self, LANDER};
use cantrip_mechanics::triggers::THIS_ENTERS;
use cantrip_oracle as oracle;

pub static CARD: CardDef = CardDef {
    abilities: &[triggered(THIS_ENTERS, tokens::create_a_lander, "When this creature enters, create a Lander token.")],
    ..oracle::GALACTIC_WAYFARER
};

pub static EXTRA: &[&CardDef] = &[&LANDER];
