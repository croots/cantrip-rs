use cantrip_api::*;
use cantrip_mechanics::{cost_reduction, effects, lands};
use cantrip_oracle as oracle;

pub static CARD: CardDef = CardDef { spell: spell(&[], effects::draw_cards_and_lose_life::<2, 2>), own_cost_changes: &[cost_reduction::less_if_you_control_subtype::<1, { lands::sub(Subtype::Villain) }>], ..oracle::VISIONS_OF_VILLAINY };
