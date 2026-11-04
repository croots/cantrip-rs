use cantrip_api::*;
use cantrip_oracle as oracle;

/// "Whenever a creature you control with a +1/+1 counter on it dies" (CR 700.4, 603.10a): looked back on, so the creature is
/// judged as it last was on the battlefield - a creature then, under the controller it last had, with a +1/+1 counter on it.
/// This creature too, if it died with one, and each of several that die together triggers it (the ruling). Tokens count.
fn creature_with_a_counter_dies(q: &dyn Query, me: &Asker, event: &Event) -> Option<Fired> {
    match event {
        Event::ZoneChange { old, from, to, .. }
            if from.is(ZoneKind::Battlefield)
                && to.is(ZoneKind::Graveyard)
                && q.obj(*old).counters(CounterKind::PlusOne) > 0
                && q.chars(*old).is(CardType::Creature)
                && q.obj(*old).controller == me.controller =>
        {
            Fired::PLAIN
        }
        _ => None,
    }
}

fn draw_a_card(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> {
    g.draw(item.controller, 1);
    Ok(())
}

pub static CARD: CardDef = CardDef {
    abilities: &[triggered(
        when_asked(EventMask::ZONE_CHANGE, creature_with_a_counter_dies),
        draw_a_card,
        "Whenever a creature you control with a +1/+1 counter on it dies, draw a card.",
    )],
    ..oracle::MELTSTRIDER_EULOGIST
};
