use cantrip_api::*;
use cantrip_mechanics::triggers::THIS_ENTERS;
use cantrip_mechanics::{effects, targets};
use cantrip_oracle as oracle;

/// "When this artifact enters, it deals 5 damage to target creature" (the target is mandatory, so the Ball's own creature
/// type is irrelevant: it is an artifact, never the target itself). "{1}{R}, Discard this card: Destroy target artifact"
/// is activated from the hand, discarding it being part of the cost (CR 118.3).
pub static CARD: CardDef = CardDef {
    abilities: &[
        triggered(THIS_ENTERS, effects::damage_first_target::<5>, "When this artifact enters, it deals 5 damage to target creature.")
            .targets(&[targets::CREATURE]),
        ability(cost("{1}{R}").and(DISCARD_THIS), effects::destroy_first_target, "{1}{R}, Discard this card: Destroy target artifact.")
            .from(ZoneKind::Hand)
            .targets(&[targets::ARTIFACT]),
    ],
    ..oracle::STEEL_WRECKING_BALL
};
