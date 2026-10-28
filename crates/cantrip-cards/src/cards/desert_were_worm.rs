use cantrip_api::*;
use cantrip_mechanics::statics::itself;
use cantrip_mechanics::triggers::you_attack_with_total_power_at_least;
use cantrip_oracle as oracle;

/// "This creature gets +2/+0 for each Mountain you control" (CR 613.4c, layer 7c): the Mountains as the earlier layers made
/// them, under the control of the Worm's controller, the Worm's own text deciding what a Mountain is (CR 612).
fn plus_two_power_per_mountain(view: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) {
    let state = view.state();
    let you = state.obj(source).controller;
    let mountain = view.words(source).subtype(Subtype::Mountain);
    let n = state.battlefield().iter().filter(|&&id| state.obj(id).controller == you && view.chars(id).has_subtype(mountain)).count() as i16;
    chars.power = chars.power.map(|v| v + 2 * n);
}

/// Whether an earlier declaration of attackers by `player` this turn had creatures with total power 12 or greater as they
/// were declared (`Attacked::chars` keeps each as it was; the declaration's `Attacks` records come together, just before its
/// `AttackersDeclared`). The history is as it was before the declaration being judged, so it holds only the earlier ones.
fn attacked_with_total_power_12_this_turn(q: &dyn Query, player: PlayerId) -> bool {
    let mut total = 0i32;
    for record in q.history().this_turn() {
        match record.event {
            Event::Attacks { attacker, known, .. } if record.active == player => {
                total += Attacked { attacker, known }.chars(q).power.map_or(0, i32::from);
            }
            Event::AttackersDeclared { player: declarer, .. } if declarer == player => {
                if total >= 12 {
                    return true;
                }
                total = 0;
            }
            _ => {}
        }
    }
    false
}

/// "Whenever you attack with creatures with total power 12 or greater for the first time each turn" (CR 603.2): the
/// declaration has the total power, and no earlier declaration this turn (the first combat's, when this is the additional
/// combat phase's) had. A trigger condition, not a limit on how often it triggers (CR 603.2d).
fn attack_with_total_power_12_for_the_first_time_each_turn(q: &dyn Query, me: &Asker, event: &Event) -> Option<Fired> {
    you_attack_with_total_power_at_least::<12>(q, me, event)?;
    if attacked_with_total_power_12_this_turn(q, me.controller) { None } else { Fired::PLAIN }
}

/// Untap all attacking creatures (CR 701.26a). After this phase, there is an additional combat phase (CR 500.8).
fn untap_attackers_and_extra_combat(g: &mut dyn Game, _: &StackItem) -> Result<(), Suspend> {
    let attacking: Vec<ObjectId> = g.state().attackers.iter().map(|a| a.id).collect();
    for creature in attacking {
        g.untap(creature);
    }
    g.add_combat_phase();
    Ok(())
}

pub static CARD: CardDef = CardDef {
    statics: &[static_ability(Layer::PtModify, itself, plus_two_power_per_mountain, "This creature gets +2/+0 for each Mountain you control.")],
    abilities: &[triggered(
        when_asked(EventMask::ATTACKERS_DECLARED, attack_with_total_power_12_for_the_first_time_each_turn),
        untap_attackers_and_extra_combat,
        "Whenever you attack with creatures with total power 12 or greater for the first time each turn, untap all attacking creatures. After this phase, there is an additional combat phase.",
    )],
    ..oracle::DESERT_WERE_WORM
};
