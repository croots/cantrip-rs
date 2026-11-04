use cantrip_api::*;
use cantrip_mechanics::{flashback, tokens};
use cantrip_oracle as oracle;

/// "Create a 2/2 green Bird creature token with 'Whenever a land you control enters, this token gets +1/+0 until end of turn.'"
/// (CR 111.1, 701.7a)
fn make_a_bird(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> {
    tokens::create_landfall_bird(g, item.controller)
}

pub static CARD: CardDef = CardDef {
    spell: spell(&[], make_a_bird),
    ways: &[flashback::way(cost("{6}{G}"), "Flashback {6}{G} (You may cast this card from your graveyard for its flashback cost. Then exile it.)")],
    ..oracle::GYSAHL_GREENS
};

pub static EXTRA: &[&CardDef] = &[&tokens::LANDFALL_BIRD];
