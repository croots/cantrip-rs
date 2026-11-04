use cantrip_api::*;
use cantrip_mechanics::{flurry, tokens};
use cantrip_oracle as oracle;

pub static CARD: CardDef = CardDef {
    abilities: &[flurry::trigger(
        "Whenever you cast your second spell each turn, create a 1/1 colorless Drone artifact creature token with flying and \"This token can block only creatures with flying.\"",
        tokens::create_a_drone,
    )],
    ..oracle::STATION_MONITOR
};

pub static EXTRA: &[&CardDef] = &[&tokens::DRONE];
