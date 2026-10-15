use cantrip_api::*;
use cantrip_oracle as oracle;

/// "Destroy all creatures" (CR 701.8): every creature goes at once, in battlefield order. A regeneration shield saves one
/// (CR 701.19a), and an indestructible one stays (CR 702.12b).
fn effect(g: &mut dyn Game, _: &StackItem) -> Result<(), Suspend> {
    let creatures: Vec<ObjectId> = g.state().battlefield().iter().copied().filter(|&id| g.chars(id).is(CardType::Creature)).collect();
    g.destroy_all(&creatures);
    Ok(())
}

pub static CARD: CardDef = CardDef {
    spell: Some(SpellDef { targets: &[], effect }),
    ..oracle::DAY_OF_JUDGMENT
};
