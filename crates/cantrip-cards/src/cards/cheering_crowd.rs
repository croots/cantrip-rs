use cantrip_api::*;
use cantrip_mechanics::effects;
use cantrip_oracle as oracle;

const ASK: Question = Question::new("PlusOneCounter", "put a +1/+1 counter on Cheering Crowd?", "If you do, you add {C} for each counter on it", "Put a counter", "Don't");

/// "At the beginning of each player's first main phase" (CR 505.1a, 603.2): any player's precombat main phase began; the ability is
/// about that player, the active one, fixed as it triggers.
fn each_players_first_main_phase(q: &dyn Query, _: ObjectId, event: &Event) -> Option<Fired> {
    match event {
        Event::StepBegan { step: Step::PrecombatMain } => Fired::about(Target::Player(q.state().turn.active)),
        _ => None,
    }
}

/// "That player may put a +1/+1 counter on this creature. If they do, they add {C} for each counter on it" (CR 122.1, 106.1, 608.2d):
/// that player decides, and a Crowd that has left the battlefield is a new object that takes no counter (CR 400.7), so there is nothing
/// to choose. The count is of counters of every kind, after the new one is on it.
fn may_put_a_counter_and_add_mana(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> {
    let Target::Player(who) = item.subject else { return Ok(()) };
    if !cantrip_mechanics::targets::permanent(g, item.object) || !effects::ask_may(g, who, item.object, Purpose::Ask(&ASK))? {
        return Ok(());
    }
    g.add_counters(item.object, CounterKind::PlusOne, 1);
    let counters = effects::counters_on(g, item.object);
    g.add_mana(who, Mana::C, counters);
    Ok(())
}

pub static CARD: CardDef = CardDef {
    abilities: &[triggered(when(EventMask::STEP_BEGAN, each_players_first_main_phase), may_put_a_counter_and_add_mana, "At the beginning of each player's first main phase, that player may put a +1/+1 counter on this creature. If they do, they add {C} for each counter on it.")],
    ..oracle::CHEERING_CROWD
};
