use cantrip_api::*;
use cantrip_mechanics::library::{look_at_top, Rest};
use cantrip_mechanics::tokens;
use cantrip_mechanics::triggers::THIS_ENTERS;
use cantrip_oracle as oracle;

/// "Look at the top five cards of your library. You may reveal up to two creature cards from among them and put them into your
/// hand. Put the rest on the bottom of your library in a random order." (CR 401.4, 701.20): with no creature card among them
/// nobody is asked.
fn look(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> {
    look_at_top(g, item.controller, item.object, 5, (0, 2), true, |c| c.is(CardType::Creature), Rest::BottomRandom)?;
    Ok(())
}

pub static CARD: CardDef = CardDef {
    abilities: &[
        triggered(
            THIS_ENTERS,
            look,
            "When this artifact enters, look at the top five cards of your library. You may reveal up to two creature cards from among them and put them into your hand. Put the rest on the bottom of your library in a random order.",
        ),
        ability(
            cost("{1}, {T}").and(SACRIFICE_THIS),
            tokens::create_a_treasure,
            "{1}, {T}, Sacrifice this artifact: Create a Treasure token.",
        ),
    ],
    ..oracle::PICTURES_OF_SPIDER_MAN
};

pub static EXTRA: &[&CardDef] = &[&tokens::TREASURE];
