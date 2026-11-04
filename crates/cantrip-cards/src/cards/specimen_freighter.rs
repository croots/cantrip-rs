use cantrip_api::*;
use cantrip_mechanics::{effects, statics, station, targets, triggers};
use cantrip_mechanics::triggers::THIS_ENTERS;
use cantrip_oracle as oracle;

const NINE_PLUS: &str = "9+ | Flying";

/// "Non-Spacecraft creature": a creature on the battlefield that does not have the Spacecraft subtype (a Spacecraft that
/// is a creature, at 9+ charge counters, is not one).
fn non_spacecraft_creature(q: &dyn Query, asker: &Asker, t: Target) -> bool {
    targets::creature_filter(q, asker, t) && matches!(t, Target::Object(id) if !q.chars(id).has_subtype(Subtype::Spacecraft))
}

pub static CARD: CardDef = CardDef {
    abilities: &[
        triggered(
            THIS_ENTERS,
            effects::return_targets_to_hand,
            "When this Spacecraft enters, return up to two target non-Spacecraft creatures to their owners' hands.",
        )
        .targets(&[up_to(2, non_spacecraft_creature)]),
        station::ability("Station (Tap another creature you control: Put charge counters equal to its power on this Spacecraft. Station only as a sorcery. It's an artifact creature at 9+.)"),
    ],
    statics: &[
        static_ability(Layer::Type, station::charged::<9>, station::is_a_creature, NINE_PLUS),
        static_ability(Layer::Ability, station::charged::<9>, station::has::<0>, NINE_PLUS).continues(),
        static_ability(Layer::Ability, station::charged::<9>, statics::have_flying, NINE_PLUS),
        static_ability(Layer::PtSet, station::charged::<9>, statics::base_pt::<4, 7>, NINE_PLUS).continues(),
    ],
    // The 9+ striation's attack trigger (CR 721.2a).
    delayed: &[triggered(
        triggers::THIS_ATTACKS,
        effects::defending_player_mills::<4>,
        "Whenever this Spacecraft attacks, defending player mills four cards.",
    )],
    ..oracle::SPECIMEN_FREIGHTER
};
