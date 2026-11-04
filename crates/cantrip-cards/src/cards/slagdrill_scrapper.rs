use cantrip_api::*;
use cantrip_mechanics::{effects, targets};
use cantrip_oracle as oracle;

/// "Draw a card" (CR 121.1).
fn draw_a_card(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> {
    effects::draw(g, item, 1)
}

/// "{2}, {T}, Sacrifice another artifact or land: Draw a card." The sacrifice is part of the cost (CR 602.2, 118.3), and "another"
/// keeps the Scrapper itself, an artifact, from paying it.
pub static CARD: CardDef = CardDef {
    abilities: &[ability(
        cost("{2}").and(TAP).and(Cost::sacrifice(1, targets::filters::another_artifact_or_land)),
        draw_a_card,
        "{2}, {T}, Sacrifice another artifact or land: Draw a card.",
    )],
    ..oracle::SLAGDRILL_SCRAPPER
};
