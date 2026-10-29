use cantrip_api::*;
use cantrip_mechanics::{effects, spent, triggers};
use cantrip_oracle as oracle;

/// Converge: "This creature enters with a +1/+1 counter on it for each color of mana spent to cast it" (CR 614.1c, 122.6). A
/// creature put onto the battlefield without being cast has had no mana spent, so it enters with none.
fn enters_with_counters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> {
    let colors = spent::colors_of_mana_spent(g, entering.object) as i16;
    entering.counters[CounterKind::PlusOne as usize] += colors;
    Ok(())
}

/// "Whenever you cast an instant or sorcery spell, creatures you control get +1/+0 until end of turn for each color of mana spent
/// to cast that spell." The spell is the one the ability triggered on (`item.subject`); its mana spent is read from its own object,
/// so the count holds even if it has since been countered (CR 608.2h). The ability resolves before the spell (CR 603.3), and
/// affects the creatures there as it resolves (CR 611.2c).
fn pump_creatures(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> {
    if let Target::Object(spell) = item.subject {
        let colors = spent::colors_of_mana_spent(g, spell) as i16;
        effects::pump_creatures_you_control(g, item, colors, 0);
    }
    Ok(())
}

pub static CARD: CardDef = CardDef {
    enters: Some(enters_with_counters),
    abilities: &[triggered(
        triggers::INSTANT_OR_SORCERY_YOU_CAST,
        pump_creatures,
        "Whenever you cast an instant or sorcery spell, creatures you control get +1/+0 until end of turn for each color of mana spent to cast that spell.",
    )],
    ..oracle::MAGMABLOOD_ARCHAIC
};
