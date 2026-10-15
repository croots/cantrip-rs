use cantrip_api::*;
use cantrip_mechanics::targets;
use cantrip_oracle as oracle;

/// "Threshold \u{2014} You can't cast this spell unless there are seven or more cards in your graveyard" (CR 601.3): judged
/// for every way of casting it. Cast from the graveyard, the card itself is not one of the seven (its ruling: there must
/// be seven other cards there).
fn threshold(q: &dyn Query, asker: &Asker) -> bool {
    q.state().zone(Zone::graveyard(asker.controller)).iter().filter(|&&card| card != asker.source).count() >= 7
}

/// "Target creature gets -3/-1 until end of turn" (CR 611.2a).
fn shrink(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> {
    if let Some(creature) = targets::object(g, item, 0) {
        g.add_effect(item.object, creature, EffectKind::Pump(-3, -1), Duration::EndOfTurn);
    }
    Ok(())
}

pub static CARD: CardDef = CardDef {
    abilities: &[AbilityDef::Activated(ActivatedDef {
        text: "{B}, Discard this card: Target creature gets -3/-1 until end of turn.",
        cost: Cost { mana: mana(0, 0, 0, 1, 0, 0), discard_self: true, ..Cost::NONE },
        targets: &[targets::CREATURE],
        effect: shrink,
        from: ZoneKind::Hand,
        ..ActivatedDef::BLANK
    })],
    cast_only_if: Some(threshold),
    ..oracle::PROFT_SINISTER_MASTERMIND
};
