use cantrip_api::*;
use cantrip_mechanics::targets::{self, is_nonland_permanent_card};
use cantrip_mechanics::{effects, triggers::THIS_ENTERS};
use cantrip_oracle as oracle;

/// "Up to one target nonland permanent card from a graveyard" (CR 115.1, 110.4a): a nonland permanent card in any player's graveyard,
/// not a spell or token.
fn nonland_permanent_card_in_a_graveyard(q: &dyn Query, _: &Asker, t: Target) -> bool {
    matches!(t, Target::Object(id) if { let o = q.obj(id); o.zone.is(ZoneKind::Graveyard) && o.became.is_none() && is_nonland_permanent_card(&q.chars(id)) })
}

/// "Exile up to one target nonland permanent and up to one target nonland permanent card from a graveyard" (CR 608.2b, 607.2a): the
/// targets still legal as it resolves are exiled together and remembered as exiled with this ability's object, for the second ability to
/// return. They are not tied to The Spot staying on the battlefield.
fn exile_the_targets(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> {
    let legal = targets::legal_objects(g, item);
    if !legal.is_empty() {
        g.exile_with(&legal, g.linked(item), false);
    }
    Ok(())
}

/// "When The Spot dies" (CR 700.4, 603.10a): looked back on; the ability is about the card in the graveyard, a new object (CR 400.7),
/// its subject.
fn this_dies(_: &dyn Query, source: ObjectId, event: &Event) -> Option<Fired> {
    match event {
        Event::ZoneChange { old, new, from, to, .. } if *old == source && from.is(ZoneKind::Battlefield) && to.is(ZoneKind::Graveyard) => {
            Fired::about(Target::Object(*new))
        }
        _ => None,
    }
}

/// "Put him on the bottom of his owner's library. If you do, return the exiled cards to their owners' hands" (CR 401.4, 608.2c): The Spot
/// only goes if he is still in the graveyard as the card that died (CR 400.7), and a replacement may send him elsewhere; the exiled cards
/// return only if he arrived in the library.
fn bottom_of_library_then_return_exiled(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> {
    let Target::Object(card) = item.subject else { return Ok(()) };
    if g.obj(card).became.is_some() { return Ok(()) }
    if g.put_into_library(&[card], LibraryPlace::Bottom).is_empty() { return Ok(()) }
    effects::return_exiled_to_hand(g, item)
}

pub static CARD: CardDef = CardDef {
    abilities: &[
        triggered(THIS_ENTERS, exile_the_targets, "When The Spot enters, exile up to one target nonland permanent and up to one target nonland permanent card from a graveyard.")
            .targets(&[targets::NONLAND_PERMANENT.optional(), target(nonland_permanent_card_in_a_graveyard).optional()]),
        triggered(when(EventMask::ZONE_CHANGE, this_dies), bottom_of_library_then_return_exiled, "When The Spot dies, put him on the bottom of his owner's library. If you do, return the exiled cards to their owners' hands."),
    ],
    ..oracle::THE_SPOT_LIVING_PORTAL
};
