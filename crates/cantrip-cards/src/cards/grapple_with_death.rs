use cantrip_api::*;
use cantrip_mechanics::{effects, targets};
use cantrip_oracle as oracle;

pub static CARD: CardDef =
    CardDef { spell: spell(&[targets::ARTIFACT_OR_CREATURE], effects::destroy_first_target_and_gain_life::<1>), ..oracle::GRAPPLE_WITH_DEATH };
