use cantrip_api::*;
use cantrip_mechanics::{effects, targets, triggers::THIS_ENTERS};
use cantrip_oracle as oracle;

/// The Omen (CR 720): cast from the hand as an Omen instant for {3}{U}; as it resolves it is shuffled into its owner's
/// library (CR 720.3d, the core's).
pub static COIL_AND_CATCH: CardDef = CardDef {
    spell: spell(&[], effects::draw_three_then_discard),
    ..oracle::COIL_AND_CATCH
};

pub static CARD: CardDef = CardDef {
    abilities: &[triggered(
        THIS_ENTERS,
        effects::return_targets_to_hand,
        "When this creature enters, return up to two other target nonland permanents to their owners' hands.",
    )
    .targets(&[up_to(2, targets::other_nonland_permanent_filter)])],
    omen: Some(&COIL_AND_CATCH),
    ..oracle::MARANG_RIVER_REGENT
};

pub static EXTRA: &[&CardDef] = &[&COIL_AND_CATCH];
