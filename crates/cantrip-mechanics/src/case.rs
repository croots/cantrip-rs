//! Case cards (CR 719.3) and solved (CR 702.169). "To solve — [condition]" means "At the beginning of your end step, if
//! [condition] and this Case is not solved, this Case becomes solved" (CR 719.3a); "Solved — [ability]" is an ability
//! that works only while the Case is solved (CR 702.169b-d, 719.3c). Case of the Crimson Pulse prints both.
//!
//! Solved is a designation (CR 719.3b), and it is asked of the event history: a permanent is solved once one of its "To
//! solve" abilities has resolved, which the core records only when the intervening "if" still held (CR 603.4). It is
//! the permanent's and not its card's: a permanent that changes zones is a new object (CR 400.7) and is not solved, a
//! copy of a solved Case is another object and is not either, and a solved Case that becomes a copy of another Case keeps
//! the record (CR 719.3b: not a copiable value).
//!
//! A Case lists `to_solve(text, condition)` among its `abilities`, `condition` asking `unsolved` as well as its own
//! condition; its solved triggered abilities match only while `solved` says so.

use cantrip_api::*;

/// The text every "To solve" ability starts with (CR 719.3a).
const TO_SOLVE: &str = "To solve \u{2014} ";

/// Whether the permanent is solved (CR 719.3b): one of its "To solve" abilities has resolved.
pub fn solved(q: &dyn Query, case: ObjectId) -> bool {
    // The core's own abilities (madness, rebound) and delayed ones are numbered past the printed list, so `get` skips them.
    q.history().select(EventQuery::kind(EventMask::RESOLVED).object(case)).any(|r| match r.event {
        Event::Resolved { ability, .. } => {
            q.card(ability.card).abilities.get(ability.index as usize).is_some_and(|a| a.text().starts_with(TO_SOLVE))
        }
        _ => false,
    })
}

/// The "this Case is not solved" half of a "To solve" ability's intervening "if" (CR 719.3a).
pub fn unsolved(q: &dyn Query, asker: &Asker) -> bool {
    !solved(q, asker.source)
}

/// It becomes solved: the record of this ability's resolution is the designation.
fn becomes_solved(_: &mut dyn Game, _: &StackItem) -> Result<(), Suspend> {
    Ok(())
}

/// "To solve — [condition]": at the beginning of your end step, if `condition` holds (it asks `unsolved` too), the Case
/// becomes solved.
pub const fn to_solve(text: &'static str, condition: TriggerCondition) -> AbilityDef {
    AbilityDef::Triggered(TriggeredDef {
        text,
        listens: EventMask::STEP_BEGAN,
        matches: crate::triggers::your_end_step,
        effect: becomes_solved,
        condition: Some(condition),
        ..TriggeredDef::BLANK
    })
}
