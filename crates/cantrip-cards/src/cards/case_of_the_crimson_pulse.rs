use cantrip_api::*;
use cantrip_mechanics::case::{solved, to_solve, unsolved};
use cantrip_mechanics::discard_chosen;
use cantrip_mechanics::triggers::{this_enters, your_upkeep};
use cantrip_oracle as oracle;

/// "Discard a card, then draw two cards": with no card in hand nothing is discarded, and the two are still drawn.
fn discard_then_draw(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> {
    discard_chosen(g, item.controller, item.object, 1)?;
    g.draw(item.controller, 2);
    Ok(())
}

/// "To solve — You have no cards in hand", and the Case is not solved yet (CR 719.3a).
fn no_cards_in_hand(q: &dyn Query, asker: &Asker) -> bool {
    unsolved(q, asker) && q.state().zone(Zone::hand(asker.controller)).is_empty()
}

/// "Solved — At the beginning of your upkeep": it triggers only if this Case is solved (CR 702.169c).
fn solved_upkeep(q: &dyn Query, source: ObjectId, event: &Event) -> Option<Fired> {
    your_upkeep(q, source, event).filter(|_| solved(q, source))
}

/// "Discard your hand, then draw two cards."
fn discard_hand_then_draw(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> {
    g.discard_hand(item.controller);
    g.draw(item.controller, 2);
    Ok(())
}

pub static CARD: CardDef = CardDef {
    abilities: &[
        AbilityDef::Triggered(TriggeredDef {
            text: "When this Case enters, discard a card, then draw two cards.",
            listens: EventMask::ZONE_CHANGE,
            matches: this_enters,
            effect: discard_then_draw,
            ..TriggeredDef::BLANK
        }),
        to_solve("To solve \u{2014} You have no cards in hand.", no_cards_in_hand),
        AbilityDef::Triggered(TriggeredDef {
            text: "Solved \u{2014} At the beginning of your upkeep, discard your hand, then draw two cards.",
            listens: EventMask::STEP_BEGAN,
            matches: solved_upkeep,
            effect: discard_hand_then_draw,
            ..TriggeredDef::BLANK
        }),
    ],
    ..oracle::CASE_OF_THE_CRIMSON_PULSE
};
