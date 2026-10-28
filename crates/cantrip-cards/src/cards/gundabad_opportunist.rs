use cantrip_api::*;
use cantrip_mechanics::library::exile_top_play_until_end_of_next_turn;
use cantrip_mechanics::triggers::THIS_ENTERS;
use cantrip_oracle as oracle;

/// "When this creature enters, exile the top card of your library. Until the end of your next turn, you may play that card."
/// An empty library exiles nothing.
fn exile_top(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> {
    exile_top_play_until_end_of_next_turn(g, item.controller, 1);
    Ok(())
}

pub static CARD: CardDef = CardDef {
    abilities: &[triggered(THIS_ENTERS, exile_top, "When this creature enters, exile the top card of your library. Until the end of your next turn, you may play that card.")],
    ..oracle::GUNDABAD_OPPORTUNIST
};
