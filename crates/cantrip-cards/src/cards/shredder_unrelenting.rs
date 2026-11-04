use cantrip_api::*;
use cantrip_mechanics::triggers::THIS_ENTERS_OR_ATTACKS;
use cantrip_mechanics::{effects, sneak, targets};
use cantrip_oracle as oracle;

pub static CARD: CardDef = CardDef {
    ways: &[sneak::way(mana("{3}{B}"), "Sneak {3}{B}")],
    abilities: &[triggered(
        THIS_ENTERS_OR_ATTACKS,
        effects::first_target_gains_deathtouch,
        "Whenever Shredder enters or attacks, another target creature you control gains deathtouch until end of turn.",
    )
    .targets(&[targets::OTHER_CREATURE_YOU_CONTROL])],
    ..oracle::SHREDDER_UNRELENTING
};
