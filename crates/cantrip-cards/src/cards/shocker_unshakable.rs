use cantrip_api::*;
use cantrip_mechanics::{statics, targets, triggers};
use cantrip_oracle as oracle;

/// "When Shocker enters, he deals 2 damage to target creature and 2 damage to that creature's controller" (CR 120.3, 608.2b): both
/// are dealt together by Shocker, as he last was if he has left (CR 113.7a), the controller being whoever controls the creature as
/// this resolves. Nothing is dealt if the creature is no longer a legal target (CR 608.2b).
fn deal_two_to_creature_and_its_controller(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> {
    if g.target_legal(item, 0) {
        let creature = item.target(0);
        let Some(object) = creature.object() else { return Ok(()) };
        let controller = Target::Player(g.controller(object));
        g.damage_together(&[(item.object, creature, 2), (item.object, controller, 2)]);
    }
    Ok(())
}

pub static CARD: CardDef = CardDef {
    statics: &[static_ability(Layer::Ability, statics::itself_during_your_turn, statics::have_first_strike, "During your turn, this creature has first strike.")],
    abilities: &[triggered(
        triggers::THIS_ENTERS,
        deal_two_to_creature_and_its_controller,
        "Vibro-Shock Gauntlets \u{2014} When Shocker enters, he deals 2 damage to target creature and 2 damage to that creature's controller.",
    )
    .targets(&[targets::CREATURE])],
    ..oracle::SHOCKER_UNSHAKABLE
};
