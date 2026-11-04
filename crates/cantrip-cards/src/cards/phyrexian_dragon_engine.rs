use cantrip_api::*;
use cantrip_mechanics::effects;
use cantrip_mechanics::unearth;
use cantrip_oracle as oracle;

const ASK: Question = Question::new("Discard", "discard your hand?", "If you do, draw three cards", "Discard, draw three", "Keep your hand");

/// "When this creature enters from your graveyard": its controller's own graveyard, not anyone else's.
fn enters_from_your_graveyard(q: &dyn Query, source: ObjectId, event: &Event) -> Option<Fired> {
    match event {
        Event::ZoneChange { new, from, to, .. }
            if *new == source && to.is(ZoneKind::Battlefield) && *from == Zone::graveyard(q.controller(source)) =>
        {
            Fired::PLAIN
        }
        _ => None,
    }
}

pub static CARD: CardDef = CardDef {
    abilities: &[
        unearth::ability(mana("{3}{R}{R}"), "Unearth {3}{R}{R}"),
        triggered(
            when(EventMask::ZONE_CHANGE, enters_from_your_graveyard),
            |g, item| effects::may_discard_hand_to_draw(g, item, &ASK, 3),
            "When this creature enters from your graveyard, you may discard your hand. If you do, draw three cards.",
        ),
    ],
    delayed: &[unearth::EXILE],
    ..oracle::PHYREXIAN_DRAGON_ENGINE
};
