use cantrip_api::*;
use cantrip_mechanics::{effects, statics, triggers};
use cantrip_oracle as oracle;

/// "If an artifact entered the battlefield under your control this turn" (CR 603.4): a move to the battlefield this turn of an
/// object that is an artifact, or was as it left the battlefield (last known information, CR 113.7a), controlled by this
/// ability's controller. Asked as it triggers and again as it resolves.
fn an_artifact_entered_under_your_control_this_turn(q: &dyn Query, asker: &Asker) -> bool {
    let entered = q.history().select(EventQuery::kind(EventMask::ZONE_CHANGE).to(ZoneKind::Battlefield).this_turn());
    entered.filter_map(|r| if let Event::ZoneChange { new, .. } = r.event { Some(new) } else { None }).any(|new| {
        let (artifact, controller) = match q.state().last_known(new) {
            Some(known) => (known.types.has(CardType::Artifact), known.controller),
            None => (q.chars(new).is(CardType::Artifact), q.controller(new)),
        };
        artifact && controller == asker.controller
    })
}

pub static CARD: CardDef = CardDef {
    abilities: &[triggered(
        triggers::THIS_ATTACKS,
        |g, item| effects::draw(g, item, 1),
        "Whenever Iron Man attacks, if an artifact entered the battlefield under your control this turn, draw a card.",
    )
    .condition(an_artifact_entered_under_your_control_this_turn)],
    statics: &[static_ability(
        Layer::PtModify,
        statics::itself,
        statics::plus_power_per_other_artifact_you_control,
        "Iron Man gets +1/+0 for each other artifact you control.",
    )],
    ..oracle::IRON_MAN_MASTER_OF_MACHINES
};
