use cantrip_api::*;
use cantrip_mechanics::effects::ask_may_return_this_card;
use cantrip_oracle as oracle;

/// "This creature has haste as long as an opponent has 10 or less life" (CR 611.3a, 613.1f): asked afresh on every
/// pass, so it has haste the moment an opponent drops to 10 and loses it if they climb back to 11.
fn opponent_at_ten_or_less(view: &dyn LayerQuery, source: ObjectId, affected: ObjectId, _: &Chars) -> bool {
    let state = view.state();
    let controller = state.obj(source).controller;
    source == affected && state.opponents(controller).any(|p| state.player(p).life <= 10)
}

fn haste(_: &dyn LayerQuery, _: ObjectId, chars: &mut Chars) {
    chars.keywords = chars.keywords.with(Keyword::Haste);
}

/// Landfall (CR 207.2c), from its owner's graveyard (CR 113.6k): the ability functions only there, and "a land you
/// control" is a land its owner controls.
fn landfall_in_graveyard(q: &dyn Query, source: ObjectId, event: &Event) -> Option<Fired> {
    let owner = q.obj(source).owner;
    if q.obj(source).zone != Zone::graveyard(owner) {
        return None;
    }
    match event {
        Event::ZoneChange { new, to, .. } if to.is(ZoneKind::Battlefield) && q.controller(*new) == owner && q.chars(*new).is(CardType::Land) => {
            Fired::PLAIN
        }
        _ => None,
    }
}

/// "You may return this card from your graveyard to the battlefield": nothing is asked once it is no longer the card that
/// triggered (CR 400.7), say exiled in response.
fn return_to_battlefield(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> {
    if ask_may_return_this_card(g, item, Purpose::Optional)? {
        g.put_onto_battlefield(item.object, item.controller, false)?;
    }
    Ok(())
}

pub static CARD: CardDef = CardDef {
    keywords: Keywords::of(&[Keyword::CantBlock]),
    statics: &[StaticDef {
        text: "This creature has haste as long as an opponent has 10 or less life.",
        layer: Layer::Ability,
        affects: opponent_at_ten_or_less,
        apply: haste,
        ..StaticDef::BLANK
    }],
    abilities: &[AbilityDef::Triggered(TriggeredDef {
        text: "Landfall \u{2014} Whenever a land you control enters, you may return this card from your graveyard to the battlefield.",
        listens: EventMask::ZONE_CHANGE,
        matches: landfall_in_graveyard,
        effect: return_to_battlefield,
        anywhere: true,
        ..TriggeredDef::BLANK
    })],
    ..oracle::BLOODGHAST
};
