use cantrip_api::*;
use cantrip_mechanics::{effects, targets};
use cantrip_oracle as oracle;

/// "Target creature has base power and toughness 1/1 until end of turn" (CR 613.4b, layer 7b): the target, if it is still
/// legal (CR 608.2b).
fn base_one_one(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> {
    effects::target_has_base_pt_until_end_of_turn(g, item, 0, 1, 1);
    Ok(())
}

/// "Target creature has base power and toughness 4/5 until end of turn".
fn base_four_five(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> {
    effects::target_has_base_pt_until_end_of_turn(g, item, 0, 4, 5);
    Ok(())
}

static MODAL: ModalDef = modes(&[
    mode(base_one_one, "Target creature has base power and toughness 1/1 until end of turn.").targets(&[targets::CREATURE]),
    mode(base_four_five, "Target creature has base power and toughness 4/5 until end of turn.").targets(&[targets::CREATURE]),
]);

pub static CARD: CardDef = CardDef {
    spell: Some(SpellDef::MODAL),
    modal: Some(&MODAL),
    ..oracle::PERFECTED_THEORY
};
