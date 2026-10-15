use cantrip_api::*;
use cantrip_mechanics::choose;
use cantrip_mechanics::triggers::PROWESS;
use cantrip_oracle as oracle;

/// "You may discard a card. If you do, draw a card" (CR 701.9, 608.2c): chosen as it resolves; choosing no card
/// declines, and the draw follows only a card really discarded.
fn improvised_act(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> {
    let hand = g.state().zone(Zone::hand(item.controller)).to_vec();
    let chosen = choose::cards(g, item.controller, item.object, Purpose::Discard, hand, 1, true)?;
    let Some(&card) = chosen.first() else { return Ok(()) };
    g.discard(card);
    g.draw(item.controller, 1);
    Ok(())
}

/// Pompous Battlemage's prepare spell (CR 722): not a card, only ever a copy.
pub static IMPROVISED_ACT: CardDef = CardDef {
    name: "Improvised Act",
    cost: mana(0, 0, 0, 0, 1, 0),
    types: TypeSet::of(&[CardType::Sorcery]),
    spell: Some(SpellDef { targets: &[], effect: improvised_act }),
    text: "You may discard a card. If you do, draw a card.",
    ..CardDef::BLANK
};

/// "This creature enters prepared" (CR 614.1d, 722.3a).
fn enters_prepared(_: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> {
    entering.prepared = true;
    Ok(())
}

pub static CARD: CardDef = CardDef {
    abilities: &[PROWESS],
    enters: Some(enters_prepared),
    prepare: Some(&IMPROVISED_ACT),
    ..oracle::POMPOUS_BATTLEMAGE
};

pub static EXTRA: &[&CardDef] = &[&IMPROVISED_ACT];
