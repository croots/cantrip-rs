use cantrip_api::*;
use cantrip_mechanics::triggers::THIS_DIES;
use cantrip_mechanics::{exile_target, targets, tokens};
use cantrip_oracle as oracle;

/// "When this creature dies, create a Treasure token." The token is made for the ability's controller, who controlled the
/// creature as it died (CR 603.3a, 603.10a).
fn treasure(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> {
    tokens::create_a_treasure(g, item)
}

/// Exile the targeted card from a graveyard (CR 701.13a); nothing if it is no longer a legal target.
fn exile_card(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> {
    exile_target(g, item, 0);
    Ok(())
}

pub static EXTRA: &[&CardDef] = &[&tokens::TREASURE];

pub static CARD: CardDef = CardDef {
    abilities: &[
        triggered(THIS_DIES, treasure, "When this creature dies, create a Treasure token."),
        ability(cost("{2}").and(TAP), exile_card, "{2}, {T}: Exile target card from a graveyard.").targets(&[targets::CARD_IN_A_GRAVEYARD]),
    ],
    ..oracle::MAGIC_POT
};
