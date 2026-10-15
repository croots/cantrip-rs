use cantrip_api::*;
use cantrip_mechanics::choose;
use cantrip_mechanics::library::{Find, search_to_hand};
use cantrip_oracle as oracle;

fn level(q: &dyn Query, source: ObjectId) -> u8 {
    q.obj(source).class_level.max(1)
}

fn at_level_one(q: &dyn Query, asker: &Asker) -> bool {
    level(q, asker.source) == 1
}

fn at_level_two(q: &dyn Query, asker: &Asker) -> bool {
    level(q, asker.source) == 2
}

fn become_level_two(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> {
    g.set_class_level(item.object, 2);
    Ok(())
}

fn become_level_three(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> {
    g.set_class_level(item.object, 3);
    Ok(())
}

/// "You may discard a card. If you do, draw a card" (CR 701.9, 608.2c): a choice made as the ability resolves, so a
/// player with no card in hand is not asked, and the draw follows only a card really discarded.
fn loot(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> {
    let hand = g.state().zone(Zone::hand(item.controller)).to_vec();
    let chosen = choose::cards(g, item.controller, item.object, Purpose::Discard, hand, 1, true)?;
    let Some(&card) = chosen.first() else { return Ok(()) };
    g.discard(card);
    g.draw(item.controller, 1);
    Ok(())
}

/// "Whenever you discard a card", at level 2 (CR 716.2c): once for each card its controller discards, from any source.
fn you_discard_at_level_two(q: &dyn Query, source: ObjectId, event: &Event) -> Option<Fired> {
    match event {
        Event::Discarded { player, .. } if *player == q.controller(source) && level(q, source) >= 2 => Fired::PLAIN,
        _ => None,
    }
}

/// "This Class deals 2 damage to each opponent."
fn two_to_each_opponent(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> {
    let opponents: Vec<PlayerId> = g.state().opponents(item.controller).collect();
    for p in opponents {
        g.damage(item.object, Target::Player(p), 2);
    }
    Ok(())
}

fn became_level_three(_: &dyn Query, source: ObjectId, event: &Event) -> Option<Fired> {
    match event {
        Event::ClassLevel { object, level: 3 } if *object == source => Fired::PLAIN,
        _ => None,
    }
}

/// "Search your library for a card, put it into your hand, shuffle, then discard a card at random" (CR 701.23, 701.9b):
/// the random discard is from the whole hand, the card just found among them.
fn tutor_then_random_discard(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> {
    search_to_hand(g, item.controller, item.object, Find::ANY, |_, _| true)?;
    cantrip_mechanics::discard_at_random(g, item.controller, 1);
    Ok(())
}

pub static CARD: CardDef = CardDef {
    abilities: &[
        AbilityDef::Triggered(TriggeredDef {
            text: "Whenever you attack, you may discard a card. If you do, draw a card.",
            listens: EventMask::ATTACKERS_DECLARED,
            matches: cantrip_mechanics::triggers::you_attack,
            effect: loot,
            ..TriggeredDef::BLANK
        }),
        AbilityDef::Activated(ActivatedDef {
            text: "{1}{R}: Level 2",
            cost: Cost::mana(mana(1, 0, 0, 0, 1, 0)),
            timing: Timing::Sorcery,
            condition: Some(at_level_one),
            effect: become_level_two,
            ..ActivatedDef::BLANK
        }),
        AbilityDef::Triggered(TriggeredDef {
            text: "Whenever you discard a card, this Class deals 2 damage to each opponent.",
            listens: EventMask::DISCARDED,
            matches: you_discard_at_level_two,
            effect: two_to_each_opponent,
            ..TriggeredDef::BLANK
        }),
        AbilityDef::Activated(ActivatedDef {
            text: "{1}{R}: Level 3",
            cost: Cost::mana(mana(1, 0, 0, 0, 1, 0)),
            timing: Timing::Sorcery,
            condition: Some(at_level_two),
            effect: become_level_three,
            ..ActivatedDef::BLANK
        }),
        AbilityDef::Triggered(TriggeredDef {
            text: "When this Class becomes level 3, search your library for a card, put it into your hand, shuffle, then discard a card at random.",
            listens: EventMask::CLASS_LEVEL,
            matches: became_level_three,
            effect: tutor_then_random_discard,
            ..TriggeredDef::BLANK
        }),
    ],
    ..oracle::COOL_BUT_RUDE
};
