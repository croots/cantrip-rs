use cantrip_api::*;
use cantrip_mechanics::triggers::{self, THIS_ENTERS};
use cantrip_mechanics::{effects, targets};
use cantrip_oracle as oracle;

/// "This creature can't be blocked this turn": for as long as it stays the same permanent (CR 611.2c, 400.7).
fn cant_be_blocked(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> {
    g.add_effect(item.object, item.object, EffectKind::CantBeBlocked, Duration::EndOfTurn);
    Ok(())
}

pub static CARD: CardDef = CardDef {
    abilities: &[
        triggered(THIS_ENTERS, effects::return_first_target_to_hand, "When this creature enters, return up to one other target creature to its owner's hand.")
            .targets(&[up_to_one(targets::other_creature_filter)]),
        triggered(triggers::YOU_CAST_A_SPELL_WITH_X_IN_ITS_MANA_COST, cant_be_blocked, "Whenever you cast a spell with {X} in its mana cost, this creature can't be blocked this turn."),
    ],
    ..oracle::MATTERBENDING_MAGE
};
