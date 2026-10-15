use cantrip_api::*;
use cantrip_mechanics::tokens;
use cantrip_oracle as oracle;

/// "Whenever you discard a card": once for each card its controller discards (CR 603.2c), whatever made them discard.
fn you_discard(q: &dyn Query, source: ObjectId, event: &Event) -> Option<Fired> {
    match event {
        Event::Discarded { player, .. } if *player == q.controller(source) => Fired::PLAIN,
        _ => None,
    }
}

/// "Choose one that hasn't been chosen this turn" (CR 700.2): a mode this Monument chose this turn, as an instance of
/// its ability went on the stack (CR 603.3c), is spent until the turn ends, from the event history. A Monument that left
/// and came back is a new object (CR 400.7). With all three spent the ability is removed as it would be put on the stack.
fn not_chosen_this_turn(q: &dyn Query, asker: &Asker, mode: u8) -> bool {
    !q.history().this_turn().iter().any(|r| {
        matches!(r.event, Event::ModesChosen { source, modes, .. } if source == asker.source && modes & (1 << mode) != 0)
    })
}

fn draw(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> {
    g.draw(item.controller, 1);
    Ok(())
}

fn treasure(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> {
    tokens::create_treasure(g, item.controller)
}

fn each_opponent_loses_three(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> {
    g.each_opponent_loses_life(item.controller, 3);
    Ok(())
}

static ENDURANCE: ModalDef = ModalDef {
    modes: &[
        ModeDef { text: "Draw a card.", targets: &[], effect: draw, cost: ManaCost::ZERO },
        ModeDef { text: "Create a Treasure token.", targets: &[], effect: treasure, cost: ManaCost::ZERO },
        ModeDef { text: "Each opponent loses 3 life.", targets: &[], effect: each_opponent_loses_three, cost: ManaCost::ZERO },
    ],
    min: 1,
    max: 1,
    escalate: None,
    repeat: false,
    allowed: Some(not_chosen_this_turn),
};

pub static CARD: CardDef = CardDef {
    abilities: &[AbilityDef::Triggered(TriggeredDef {
        text: "Whenever you discard a card, choose one that hasn't been chosen this turn \u{2014} draw a card; or create a Treasure token; or each opponent loses 3 life.",
        listens: EventMask::DISCARDED,
        matches: you_discard,
        modal: Some(&ENDURANCE),
        ..TriggeredDef::BLANK
    })],
    ..oracle::MONUMENT_TO_ENDURANCE
};

pub static EXTRA: &[&CardDef] = &[&tokens::TREASURE];
