use cantrip_api::*;
use cantrip_mechanics::modified::MODIFIED_CREATURE_YOU_CONTROL;
use cantrip_mechanics::triggers::{THIS_ATTACKS, THIS_ENTERS};
use cantrip_mechanics::{effects, targets};
use cantrip_oracle as oracle;

/// "Target modified creature you control gains lifelink until end of turn" (CR 700.9, 702.15, 611.2a). The target is chosen as the
/// ability goes on the stack, so it must be modified then; once chosen it gains lifelink if it is still a legal target as this
/// resolves (CR 608.2b), modified or not by then.
fn modified_creature_gains_lifelink(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> {
    if let Some(creature) = targets::object(g, item, 0) {
        effects::gain_lifelink_until_end_of_turn(g, item.object, creature);
    }
    Ok(())
}

pub static CARD: CardDef = CardDef {
    abilities: &[
        triggered(THIS_ENTERS, effects::plus_one_counter_on_target, "When Silver Sable enters, put a +1/+1 counter on another target creature.")
            .targets(&[targets::OTHER_CREATURE]),
        triggered(
            THIS_ATTACKS,
            modified_creature_gains_lifelink,
            "Whenever Silver Sable attacks, target modified creature you control gains lifelink until end of turn.",
        )
        .targets(&[MODIFIED_CREATURE_YOU_CONTROL]),
    ],
    ..oracle::SILVER_SABLE_MERCENARY_LEADER
};
