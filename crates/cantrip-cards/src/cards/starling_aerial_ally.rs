use cantrip_api::*;
use cantrip_mechanics::{effects, targets, triggers::THIS_ENTERS};
use cantrip_oracle as oracle;

/// "Another target creature you control gains flying until end of turn" (CR 611.2a, 702.9b): nothing if the target is no longer
/// legal when it resolves (CR 608.2b).
fn gain_flying(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> {
    let Some(creature) = targets::object(g, item, 0) else { return Ok(()) };
    effects::gain_until_end_of_turn(g, item.object, creature, Keyword::Flying);
    Ok(())
}

// Flying is the core's, read from the card's JSON.
pub static CARD: CardDef = CardDef {
    abilities: &[triggered(THIS_ENTERS, gain_flying, "When Starling enters, another target creature you control gains flying until end of turn.")
        .targets(&[targets::OTHER_CREATURE_YOU_CONTROL])],
    ..oracle::STARLING_AERIAL_ALLY
};
