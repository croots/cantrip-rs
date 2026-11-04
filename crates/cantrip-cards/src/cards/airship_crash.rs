use cantrip_api::*;
use cantrip_mechanics::{destroy_target, targets, triggers};
use cantrip_oracle as oracle;

/// "Destroy target artifact, enchantment, or creature with flying." (CR 701.8a.)
fn destroy_it(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> {
    destroy_target(g, item, 0);
    Ok(())
}

pub static CARD: CardDef = CardDef {
    spell: spell(&[targets::ARTIFACT_ENCHANTMENT_OR_CREATURE_WITH_FLYING], destroy_it),
    abilities: &[triggers::cycling(mana("{2}"), "Cycling {2} ({2}, Discard this card: Draw a card.)")],
    ..oracle::AIRSHIP_CRASH
};
