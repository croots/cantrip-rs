use cantrip_api::*;
use cantrip_mechanics::effects;
use cantrip_oracle as oracle;

/// Whether `id` was a creature on the battlefield, under `player`, immediately before the event being looked for (CR 603.10a): on it
/// now, or one that left in that same event. Its old row still says it was on the battlefield (CR 400.7) and its leaving is not yet in
/// the history, which ends before the events whose triggers are being looked for; one that left earlier is in it, and no longer counts.
fn was_their_creature(q: &dyn Query, id: ObjectId, player: PlayerId) -> bool {
    let row = q.obj(id);
    row.controller == player
        && row.zone.is(ZoneKind::Battlefield)
        && q.chars(id).is(CardType::Creature)
        && (row.became.is_none() || !q.history().all().iter().any(|r| matches!(r.event, Event::ZoneChange { old, .. } if old == id)))
}

/// "Whenever a creature an opponent controls with the greatest power among creatures that player controls dies" (CR 700.4, 603.10a,
/// 208.1): the creature as it last was, under the controller it last had, an opponent of this ability's controller, with no creature
/// that player controlled just before having more power, counting those that died with it. Creatures tied for the greatest power each
/// qualify, so each triggers it.
fn greatest_power_creature_an_opponent_controls_dies(q: &dyn Query, me: &Asker, event: &Event) -> Option<Fired> {
    let Event::ZoneChange { old, from, to, .. } = event else { return None };
    if !from.is(ZoneKind::Battlefield) || !to.is(ZoneKind::Graveyard) || !q.chars(*old).is(CardType::Creature) { return None }
    let player = q.obj(*old).controller;
    if !q.state().opponents(me.controller).any(|p| p == player) { return None }
    let power = q.chars(*old).power?;
    let beaten = (0..q.state().objects.len() as u32).map(ObjectId).any(|id| was_their_creature(q, id, player) && q.chars(id).power.is_some_and(|p| p > power));
    if beaten { None } else { Fired::PLAIN }
}

/// "Draw a card and put a +1/+1 counter on Kraven the Hunter" (CR 121.1, 122.1).
fn draw_and_grow(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> {
    g.draw(item.controller, 1);
    effects::plus_one_counter_on_this(g, item)
}

/// Trample is a core keyword.
pub static CARD: CardDef = CardDef {
    abilities: &[triggered(
        when_asked(EventMask::ZONE_CHANGE, greatest_power_creature_an_opponent_controls_dies),
        draw_and_grow,
        "Whenever a creature an opponent controls with the greatest power among creatures that player controls dies, draw a card and put a +1/+1 counter on Kraven the Hunter.",
    )],
    ..oracle::KRAVEN_THE_HUNTER
};
