use cantrip_api::*;
use cantrip_mechanics::effects::plus_one_counter_on_this;
use cantrip_mechanics::history;
use cantrip_mechanics::targets;
use cantrip_mechanics::triggers::{THIS_ENTERS, YOUR_END_STEP};
use cantrip_oracle as oracle;

/// "Target artifact or creature card from your graveyard" (CR 115.1): a card of either type in its controller's own graveyard.
fn artifact_or_creature_card_in_your_graveyard(q: &dyn Query, asker: &Asker, t: Target) -> bool {
    targets::card_in_your_graveyard(q, asker, t, &[CardType::Artifact, CardType::Creature])
}

/// "Return target artifact or creature card from your graveyard to your hand." (CR 608.2b: a target that left the graveyard is
/// illegal, and the ability then does nothing.)
fn return_it(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> {
    let Some(card) = targets::object(g, item, 0) else { return Ok(()) };
    g.return_to_hand(&[card]);
    Ok(())
}

/// Flying is in the content JSON.
pub static CARD: CardDef = CardDef {
    abilities: &[
        triggered(
            THIS_ENTERS,
            return_it,
            "When this creature enters, return target artifact or creature card from your graveyard to your hand.",
        )
        .targets(&[target(artifact_or_creature_card_in_your_graveyard)]),
        // "Void" (CR 207.2c, 603.4): an intervening "if", checked as the end step begins and again as the ability resolves.
        triggered(
            YOUR_END_STEP,
            plus_one_counter_on_this,
            "Void \u{2014} At the beginning of your end step, if a nonland permanent left the battlefield this turn or a spell was warped this turn, put a +1/+1 counter on this creature.",
        )
        .condition(history::void_condition),
    ],
    ..oracle::INTERCEPTOR_MECHAN
};
