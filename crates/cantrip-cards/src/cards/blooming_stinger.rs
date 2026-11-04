use cantrip_api::*;
use cantrip_mechanics::triggers::THIS_ENTERS;
use cantrip_mechanics::{effects, targets};
use cantrip_oracle as oracle;

pub static CARD: CardDef = CardDef {
    abilities: &[triggered(
        THIS_ENTERS,
        effects::first_target_gains_deathtouch,
        "When this creature enters, another target creature you control gains deathtouch until end of turn.",
    )
    .targets(&[targets::OTHER_CREATURE_YOU_CONTROL])],
    ..oracle::BLOOMING_STINGER
};
