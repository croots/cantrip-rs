use cantrip_api::*;
use cantrip_mechanics::triggers::{self, YOU_ATTACK};
use cantrip_oracle as oracle;

const PAY_LIFE: Question = Question::new(
    "ToughnessDamage",
    "pay {cost} so your creatures assign combat damage by toughness?",
    "Until end of turn, creatures you control with toughness greater than their power assign combat damage equal to their toughness",
    "Pay {cost}",
    "Don't pay",
);

/// "Whenever you attack, you may pay 2 life. If you do, until end of turn, creatures you control with toughness greater than
/// their power assign combat damage equal to their toughness rather than their power" (CR 119.4, 510.1a). A player with less
/// than 2 life can't pay it and isn't asked.
fn attack(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> {
    let you = item.controller;
    if g.state().player(you).life >= 2 && g.may_pay(you, item.object, item.object, life(2), &PAY_LIFE)? {
        g.add_player_effect(you, item.object, EffectKind::CreaturesAssignDamageByToughness, Duration::EndOfTurn);
    }
    Ok(())
}

pub static CARD: CardDef = CardDef {
    abilities: &[
        triggers::extort("Extort (Whenever you cast a spell, you may pay {W/B}. If you do, each opponent loses 1 life and you gain that much life.)"),
        triggered(
            YOU_ATTACK,
            attack,
            "Whenever you attack, you may pay 2 life. If you do, until end of turn, creatures you control with toughness greater than their power assign combat damage equal to their toughness rather than their power.",
        ),
    ],
    ..oracle::THE_KINGPIN_OF_CRIME
};
