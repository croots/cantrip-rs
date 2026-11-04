use cantrip_api::*;
use cantrip_mechanics::{effects, targets};
use cantrip_oracle as oracle;

pub static CARD: CardDef = CardDef {
    spell: spell(&[target(targets::permanent_with_mana_value_at_least::<4>)], effects::exile_first_target),
    ..oracle::DESPARK
};
