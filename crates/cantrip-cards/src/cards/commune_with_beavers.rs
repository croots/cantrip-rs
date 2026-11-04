use cantrip_api::*;
use cantrip_mechanics::library::{look_at_top, Rest};
use cantrip_oracle as oracle;

/// "Look at the top three cards of your library. You may reveal an artifact, creature, or land card from among them and put it
/// into your hand. Put the rest on the bottom of your library in any order." (CR 401.4, 701.20a)
fn look(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> {
    let wanted = |c: &Chars| c.is(CardType::Artifact) || c.is(CardType::Creature) || c.is(CardType::Land);
    look_at_top(g, item.controller, item.object, 3, (0, 1), true, wanted, Rest::BottomChosen)?;
    Ok(())
}

pub static CARD: CardDef = CardDef { spell: spell(&[], look), ..oracle::COMMUNE_WITH_BEAVERS };
