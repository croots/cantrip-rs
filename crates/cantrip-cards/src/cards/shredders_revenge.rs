use cantrip_api::*;
use cantrip_mechanics::{discard_chosen, effects, targets};
use cantrip_oracle as oracle;

/// Mode 1: "Target player discards two cards." (CR 701.9): their choice, discarded together; nothing happens if the target
/// is no longer legal (CR 608.2b).
fn discard_two(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> {
    let Some(player) = targets::player(g, item, 0) else { return Ok(()) };
    discard_chosen(g, player, item.object, 2)?;
    Ok(())
}

static MODAL: ModalDef = modes(&[
    mode(discard_two, "Target player discards two cards.").targets(&[targets::PLAYER]),
    mode(effects::target_player_draws_and_loses_life::<2, 2>, "Target player draws two cards and loses 2 life.").targets(&[targets::PLAYER]),
]);

pub static CARD: CardDef = CardDef { spell: Some(SpellDef::MODAL), modal: Some(&MODAL), ..oracle::SHREDDERS_REVENGE };
