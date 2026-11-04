use cantrip_api::*;
use cantrip_mechanics::{cost_reduction, each_discards_a_card, hand, targets, triggers};
use cantrip_oracle as oracle;

/// "When this creature enters, any number of target opponents each discard a card. For each of those opponents who didn't
/// discard a card with mana value 4 or greater, draw a card." Each target still legal as it resolves (CR 608.2b) chooses a
/// card in turn order from the active player and they are discarded together (CR 101.4, 701.9); an opponent with no card
/// in hand discarded none, so you draw for them.
fn discard_then_draw(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> {
    let legal = targets::legal_players(g, item);
    let opponents: Vec<PlayerId> = g.state().apnap().filter(|p| legal.contains(p)).collect();
    let discarded = each_discards_a_card(g, &opponents, item.object)?;
    // The cards are in graveyards now, so their mana values are those of the cards (CR 202.3); one a replacement put into a
    // hidden zone unrevealed is undefined and has none (CR 701.9c, `hand::discarded_chars`).
    let big = |g: &dyn Game, p: PlayerId| {
        discarded.iter().any(|&c| g.state().obj(c).owner == p && hand::discarded_chars(g, c).is_some() && g.mana_value(c) >= 4)
    };
    let draws = opponents.iter().filter(|&&p| !big(g, p)).count();
    g.draw(item.controller, draws as u32);
    Ok(())
}

pub static CARD: CardDef = CardDef {
    own_cost_changes: &[cost_reduction::for_each_creature_card_in_your_graveyard],
    abilities: &[triggered(
        triggers::THIS_ENTERS,
        discard_then_draw,
        "When this creature enters, any number of target opponents each discard a card. For each of those opponents who didn't discard a card with mana value 4 or greater, draw a card.",
    )
    .targets(&[targets::OPPONENT.between(0, TargetSpec::ANY_NUMBER).labelled("Opponents who each discard a card")])],
    ..oracle::HOLLOW_MARAUDER
};
