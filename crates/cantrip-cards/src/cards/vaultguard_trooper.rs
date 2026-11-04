use cantrip_api::*;
use cantrip_mechanics::{effects, triggers};
use cantrip_oracle as oracle;

const ASK: Question = Question::new("Discard", "discard your hand?", "If you do, draw two cards", "Discard, draw two", "Keep your hand");

pub static CARD: CardDef = CardDef {
    abilities: &[triggered(
        triggers::YOUR_END_STEP,
        |g, item| effects::may_discard_hand_to_draw(g, item, &ASK, 2),
        "At the beginning of your end step, if you control two or more tapped creatures, you may discard your hand. If you do, draw two cards.",
    )
    .condition(triggers::you_control_two_or_more_tapped_creatures)],
    ..oracle::VAULTGUARD_TROOPER
};
