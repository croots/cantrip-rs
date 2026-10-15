use cantrip_api::*;
use cantrip_mechanics::{choose_from_hand, effects, targets, triggers::this_enters};
use cantrip_oracle as oracle;

const STUN_TEXT: &str = "When you discard one or more nonland cards this way, tap up to that many target creatures and put a stun counter on each of them.";

/// Draw two cards, then discard two (all of a shorter hand) as one action (CR 701.9b). A card's type is read before it
/// moves (CR 400.7); one or more nonland cards discarded this way trigger the reflexive ability (CR 603.12), whose
/// "up to that many" is fixed now: its version with up to one target, or with up to two.
fn enter(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> {
    let you = item.controller;
    g.draw(you, 2);
    let chosen = choose_from_hand(g, you, item.object, 2)?;
    let nonlands = chosen.iter().filter(|&&c| !g.chars(c).is(CardType::Land)).count();
    g.discard_together(&chosen);
    if nonlands > 0 {
        let this = g.card_id(&CARD);
        g.reflexive_trigger(item.object, you, AbilityRef::delayed(this, nonlands as u8 - 1), Target::None);
    }
    Ok(())
}

/// Tap each target creature still legal and put a stun counter on it (CR 122.1d); an already tapped one still gets
/// its counter.
fn tap_and_stun(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> {
    for creature in targets::legal_objects(g, item) {
        g.tap(creature);
        g.add_counters(creature, CounterKind::Stun, 1);
    }
    Ok(())
}

pub static CARD: CardDef = CardDef {
    abilities: &[
        AbilityDef::Triggered(TriggeredDef {
            text: "When this creature enters, draw two cards, then discard two cards. When you discard one or more nonland cards this way, tap up to that many target creatures and put a stun counter on each of them.",
            listens: EventMask::ZONE_CHANGE,
            matches: this_enters,
            effect: enter,
            ..TriggeredDef::BLANK
        }),
        AbilityDef::Activated(ActivatedDef {
            text: "{3}{U}{U}, Exile this card from your graveyard: Draw two cards.",
            cost: Cost { mana: mana(3, 0, 2, 0, 0, 0), exile_self: true, ..Cost::NONE },
            effect: effects::draw::<2>,
            from: ZoneKind::Graveyard,
            ..ActivatedDef::BLANK
        }),
    ],
    delayed: &[
        AbilityDef::Triggered(TriggeredDef {
            text: STUN_TEXT,
            targets: &[TargetSpec { min: 0, max: 1, filter: targets::CREATURE.filter }],
            effect: tap_and_stun,
            ..TriggeredDef::BLANK
        }),
        AbilityDef::Triggered(TriggeredDef {
            text: STUN_TEXT,
            targets: &[TargetSpec { min: 0, max: 2, filter: targets::CREATURE.filter }],
            effect: tap_and_stun,
            ..TriggeredDef::BLANK
        }),
    ],
    ..oracle::SEASONED_CRYOMANCER
};
