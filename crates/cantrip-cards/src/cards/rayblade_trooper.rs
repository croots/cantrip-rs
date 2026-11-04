use cantrip_api::*;
use cantrip_mechanics::triggers::THIS_ENTERS;
use cantrip_mechanics::{effects, targets, tokens, warp};
use cantrip_oracle as oracle;

/// "Whenever a nontoken creature you control with a +1/+1 counter on it dies" (CR 700.4, 603.10a): looked back on, so the
/// creature is judged as it last was on the battlefield - a nontoken creature then, under the controller it last had, with a
/// +1/+1 counter on it. This creature too, if it died with one, and each of several that die together triggers it (its ruling).
fn nontoken_creature_with_a_counter_dies(q: &dyn Query, me: &Asker, event: &Event) -> Option<Fired> {
    match event {
        Event::ZoneChange { old, from, to, .. }
            if from.is(ZoneKind::Battlefield)
                && to.is(ZoneKind::Graveyard)
                && !q.obj(*old).token
                && q.obj(*old).counters(CounterKind::PlusOne) > 0
                && q.chars(*old).is(CardType::Creature)
                && q.obj(*old).controller == me.controller =>
        {
            Fired::PLAIN
        }
        _ => None,
    }
}

/// Warp is a keyword: `warp::way`, with its end-step exile and recast (CR 702.185).
pub static CARD: CardDef = CardDef {
    abilities: &[
        triggered(THIS_ENTERS, effects::plus_one_counter_on_target, "When this creature enters, put a +1/+1 counter on target creature you control.")
            .targets(&[targets::CREATURE_YOU_CONTROL]),
        triggered(
            when_asked(EventMask::ZONE_CHANGE, nontoken_creature_with_a_counter_dies),
            tokens::create_a_human_soldier,
            "Whenever a nontoken creature you control with a +1/+1 counter on it dies, create a 1/1 white Human Soldier creature token.",
        ),
    ],
    ways: &[warp::way(mana("{1}{W}"), "Warp {1}{W}")],
    end_step_riders: warp::RIDERS,
    delayed: &[warp::EXILE],
    ..oracle::RAYBLADE_TROOPER
};

pub static EXTRA: &[&CardDef] = &[&tokens::HUMAN_SOLDIER];
