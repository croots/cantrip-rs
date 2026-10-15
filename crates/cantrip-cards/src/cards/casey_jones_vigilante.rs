use cantrip_api::*;
use cantrip_oracle as oracle;

/// "Draw three cards. At the beginning of your next upkeep, discard three cards at random": the delayed trigger (CR
/// 603.7) is created as the ability resolves and belongs to no permanent, so it happens whether or not Casey Jones is
/// still around; it waits for its controller's upkeep, not the next upkeep of anyone's turn.
fn draw_then_later_discard(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> {
    g.draw(item.controller, 3);
    let card = g.obj(item.object).card;
    g.delayed_trigger(
        DelayedWhen::Step { step: Step::Upkeep, on_turn_of: Some(item.controller) },
        item.object,
        item.controller,
        AbilityRef::delayed(card, 0),
        Target::None,
    );
    Ok(())
}

/// "Discard three cards at random" (CR 701.9b): a smaller hand is discarded whole.
fn discard_three_at_random(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> {
    cantrip_mechanics::discard_at_random(g, item.controller, 3);
    Ok(())
}

pub static CARD: CardDef = CardDef {
    abilities: &[AbilityDef::Triggered(TriggeredDef {
        text: "When Casey Jones enters, draw three cards. At the beginning of your next upkeep, discard three cards at random.",
        listens: EventMask::ZONE_CHANGE,
        matches: cantrip_mechanics::triggers::this_enters,
        effect: draw_then_later_discard,
        ..TriggeredDef::BLANK
    })],
    delayed: &[AbilityDef::Triggered(TriggeredDef {
        text: "Discard three cards at random.",
        effect: discard_three_at_random,
        ..TriggeredDef::BLANK
    })],
    ..oracle::CASEY_JONES_VIGILANTE
};
