use cantrip_api::*;
use cantrip_mechanics::targets;
use cantrip_mechanics::targets::filters;
use cantrip_oracle as oracle;

/// "This creature gains lifelink until end of turn": nothing to give it to once it has left.
fn gain_lifelink(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> {
    if targets::permanent(g, item.object) {
        g.add_effect(item.object, item.object, EffectKind::AddKeywords(Keywords::of(&[Keyword::Lifelink])), Duration::EndOfTurn);
    }
    Ok(())
}

/// "Whenever one or more cards leave your graveyard" (CR 603.2c, 603.10a): once for the cards that leave together, from
/// the graveyard of this creature's controller, wherever they go.
fn cards_leave_your_graveyard(q: &dyn Query, source: ObjectId, event: &Event) -> Option<Fired> {
    match event {
        Event::ZoneChange { from, .. } if *from == Zone::graveyard(q.controller(source)) => Fired::PLAIN,
        _ => None,
    }
}

/// "Put a +1/+1 counter on target creature you control."
fn counter(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> {
    if let Some(creature) = targets::object(g, item, 0) {
        g.add_counters(creature, CounterKind::PlusOne, 1);
    }
    Ok(())
}

pub static CARD: CardDef = CardDef {
    abilities: &[
        AbilityDef::Activated(ActivatedDef {
            text: "Discard a card: This creature gains lifelink until end of turn.",
            cost: Cost { discard: Some((1, filters::any)), ..Cost::NONE },
            effect: gain_lifelink,
            ..ActivatedDef::BLANK
        }),
        AbilityDef::Triggered(TriggeredDef {
            text: "Whenever one or more cards leave your graveyard, put a +1/+1 counter on target creature you control.",
            listens: EventMask::ZONE_CHANGE,
            matches: cards_leave_your_graveyard,
            targets: &[targets::CREATURE_YOU_CONTROL],
            effect: counter,
            once_per_batch: true,
            ..TriggeredDef::BLANK
        }),
    ],
    ..oracle::HARDENED_ACADEMIC
};
