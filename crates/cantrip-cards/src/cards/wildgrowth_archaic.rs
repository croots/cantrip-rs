use cantrip_api::*;
use cantrip_mechanics::spent;
use cantrip_mechanics::triggers::CREATURE_SPELL_YOU_CAST;
use cantrip_oracle as oracle;

/// Converge (an ability word) — "This creature enters with a +1/+1 counter on it for each color of mana spent to cast it"
/// (CR 207.2c, 614.1c, 601.2h): colorless is not a color, and put onto the battlefield without being cast, or as a copy,
/// none was spent.
fn enters_with_a_counter_per_color(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> {
    entering.counters[CounterKind::PlusOne as usize] += spent::colors_of_mana_spent(g, entering.object) as i16;
    Ok(())
}

/// "Whenever you cast a creature spell, that creature enters with X additional +1/+1 counters on it": the spell rides along as
/// the trigger's subject, and the trigger makes a replacement effect about it for the turn, applied as it enters the
/// battlefield from the stack (CR 614.12), where X is read from the spell itself.
fn give_the_spell_counters(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> {
    if let Target::Object(spell) = item.subject {
        g.add_effect(item.object, spell, EffectKind::Replacement(0), Duration::EndOfTurn);
    }
    Ok(())
}

fn spell_enters(_: &dyn Query, _: &Asker, affected: ObjectId, proposal: &Proposal) -> bool {
    matches!(proposal, Proposal::ZoneChange(z) if z.object == affected && z.from.is(ZoneKind::Stack) && z.to.is(ZoneKind::Battlefield))
}

/// X, "the number of colors of mana spent to cast it", of the creature spell that is entering.
fn add_counters_per_color(q: &dyn Query, _: &Asker, affected: ObjectId, proposal: &mut Proposal) {
    if let Proposal::ZoneChange(z) = proposal {
        z.counters[CounterKind::PlusOne as usize] += spent::colors_of_mana_spent(q, affected) as i16;
    }
}

pub static CARD: CardDef = CardDef {
    enters: Some(enters_with_a_counter_per_color),
    abilities: &[triggered(
        CREATURE_SPELL_YOU_CAST,
        give_the_spell_counters,
        "Whenever you cast a creature spell, that creature enters with X additional +1/+1 counters on it, where X is the number of colors of mana spent to cast it.",
    )],
    granted_replacements: &[granted_replacement(
        ReplacementKind::ZoneChange,
        spell_enters,
        add_counters_per_color,
        "enters with X additional +1/+1 counters, where X is the number of colors of mana spent to cast it",
    )],
    ..oracle::WILDGROWTH_ARCHAIC
};
