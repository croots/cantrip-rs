use cantrip_api::*;
use cantrip_mechanics::targets;
use cantrip_oracle as oracle;

/// Target creature card with mana value 3 or less from your graveyard: a card there with {X} in its cost counts X as 0
/// (CR 107.3m, 202.3e).
fn small_creature_in_your_graveyard(q: &dyn Query, asker: &Asker, t: Target) -> bool {
    targets::card_in_your_graveyard(q, asker, t, &[CardType::Creature]) && matches!(t, Target::Object(id) if q.mana_value(id) <= 3)
}

/// Return target creature card with mana value 3 or less from your graveyard to the battlefield tapped (CR 110.5b).
fn effect(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> {
    // A spell whose only target is gone does not resolve at all (CR 608.2b), so the card is always here.
    let Some(card) = targets::object(g, item, 0) else { return Ok(()) };
    g.put_onto_battlefield(card, item.controller, true)?;
    Ok(())
}

pub static CARD: CardDef = CardDef {
    spell: Some(SpellDef { targets: &[TargetSpec { min: 1, max: 1, filter: small_creature_in_your_graveyard }], effect }),
    ..oracle::HELPING_HAND
};
