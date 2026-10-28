use cantrip_api::*;
use cantrip_mechanics::effects;
use cantrip_oracle as oracle;

/// "Whenever this creature becomes the target of a spell or ability an opponent controls" (CR 603.2, 115.10): each time it is
/// made a target is its own event, so a spell naming it twice cannot (CR 115.3) but two abilities each do. Only a spell or ability
/// one of this creature's controller's opponents controls counts, one its controller controls never.
fn this_targeted_by_an_opponent(q: &dyn Query, me: &Asker, event: &Event) -> Option<Fired> {
    match event {
        Event::BecameTarget { target: Target::Object(t), controller, .. }
            if *t == me.source && q.state().opponents(me.controller).any(|p| p == *controller) =>
        {
            Fired::PLAIN
        }
        _ => None,
    }
}

pub static CARD: CardDef = CardDef {
    // "This creature can't be blocked by creatures with power 2 or less" (CR 509.1b): checked as blockers are declared, with the
    // power the blocker has then, and no matter what happens to it afterwards.
    keywords: Keywords::of(&[Keyword::Reach, Keyword::CantBeBlockedByPowerTwoOrLess]),
    abilities: &[triggered(
        when_asked(EventMask::BECAME_TARGET, this_targeted_by_an_opponent),
        |g, item| effects::draw(g, item, 1),
        "Whenever this creature becomes the target of a spell or ability an opponent controls, draw a card.",
    )],
    ..oracle::OLD_FAT_SPIDER
};
