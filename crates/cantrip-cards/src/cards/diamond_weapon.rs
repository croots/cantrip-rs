use cantrip_api::*;
use cantrip_mechanics::cost_reduction;
use cantrip_oracle as oracle;

/// "Immune — Prevent all combat damage that would be dealt to Diamond Weapon." (CR 615.1, 510.2): applies to combat damage
/// dealt to this object by anything; the prevention is not considered as a trampling attacker assigns its damage (CR 702.19b), so
/// it must still assign lethal damage to it.
fn prevents_combat_damage_to_this(_: &dyn Query, asker: &Asker, e: &DamageEvent) -> bool {
    e.combat && matches!(e.target, Target::Object(o) if o == asker.source)
}

pub static CARD: CardDef = CardDef {
    own_cost_changes: &[cost_reduction::for_each_permanent_card_in_your_graveyard],
    preventions: &[PreventionDef {
        hint: "prevents all combat damage that would be dealt to this creature",
        applies: prevents_combat_damage_to_this,
        amount: None,
    }],
    ..oracle::DIAMOND_WEAPON
};
