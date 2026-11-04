use cantrip_api::*;
use cantrip_mechanics::triggers::THIS_ENTERS;
use cantrip_mechanics::{artifacts_you_control, effects, targets};
use cantrip_oracle as oracle;

/// "When this creature enters, until end of turn, target creature you control gets +X/+0 and target creature an opponent controls
/// gets -0/-X, where X is the number of artifacts you control" (CR 611.2a, 608.2h): X is worked out once, as the ability
/// resolves, the Circuit-Feeder itself among the artifacts; a target that is no longer legal is left alone while the other
/// still gets its half (CR 608.2b). Flying is in the content JSON.
fn pump_and_shrink(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> {
    let x = artifacts_you_control(g, item.controller) as i16;
    effects::first_target_gets_plus_x_second_gets_minus_x(g, item, x)
}

pub static CARD: CardDef = CardDef {
    abilities: &[triggered(
        THIS_ENTERS,
        pump_and_shrink,
        "When this creature enters, until end of turn, target creature you control gets +X/+0 and target creature an opponent controls gets -0/-X, where X is the number of artifacts you control.",
    )
    .targets(&[targets::CREATURE_YOU_CONTROL, targets::CREATURE_AN_OPPONENT_CONTROLS])],
    ..oracle::MONOIST_CIRCUIT_FEEDER
};
