use cantrip_api::*;
use cantrip_mechanics::targets;
use cantrip_oracle as oracle;

/// An outlaw (CR 700.12): an object with the Assassin, Mercenary, Pirate, Rogue and/or Warlock creature type.
const OUTLAW_TYPES: [Subtype; 5] = [Subtype::Assassin, Subtype::Mercenary, Subtype::Pirate, Subtype::Rogue, Subtype::Warlock];

/// "Target non-outlaw creature" (CR 115.1, 700.12): any creature on the battlefield with none of the outlaw types, as it
/// is now (asked again as the spell resolves, CR 608.2b).
fn non_outlaw_creature(q: &dyn Query, asker: &Asker, t: Target) -> bool {
    (targets::CREATURE.filter)(q, asker, t)
        && matches!(t, Target::Object(id) if { let chars = q.chars(id); !OUTLAW_TYPES.iter().any(|&s| chars.has_subtype(s)) })
}

/// "Destroy target non-outlaw creature" (CR 701.8a).
fn shoot(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> {
    if let Some(creature) = targets::object(g, item, 0) {
        g.destroy(creature);
    }
    Ok(())
}

pub static CARD: CardDef = CardDef {
    spell: Some(SpellDef { targets: &[TargetSpec { min: 1, max: 1, filter: non_outlaw_creature }], effect: shoot }),
    ..oracle::SHOOT_THE_SHERIFF
};
