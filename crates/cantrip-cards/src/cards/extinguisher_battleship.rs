use cantrip_api::*;
use cantrip_mechanics::{destroy_target, effects, statics, station, targets, triggers};
use cantrip_oracle as oracle;

const FIVE_PLUS: &str = "5+ | Flying, trample";

/// "When this Spacecraft enters, destroy target noncreature permanent. Then this Spacecraft deals 4 damage to each creature."
/// (CR 701.8, 120.3, 608.2b-c): the one target is a noncreature permanent, so with it illegal the ability does not resolve and no
/// creature is dealt damage; the damage follows the destruction, as a new sentence, and is dealt to every creature on the
/// battlefield at that moment, a creature the destroyed permanent was included (CR 608.2c).
fn destroy_then_deal_4_damage_to_each_creature(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> {
    destroy_target(g, item, 0);
    effects::damage_each_creature(g, item, 4);
    Ok(())
}

/// The 5+ striation's keywords (CR 721.2a): "Flying, trample" in the one layer-6 effect.
fn have_flying_and_trample(view: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) {
    statics::have_flying(view, source, chars);
    statics::have_trample(view, source, chars);
}

pub static CARD: CardDef = CardDef {
    abilities: &[
        triggered(
            triggers::THIS_ENTERS,
            destroy_then_deal_4_damage_to_each_creature,
            "When this Spacecraft enters, destroy target noncreature permanent. Then this Spacecraft deals 4 damage to each creature.",
        )
        .targets(&[targets::NONCREATURE_PERMANENT]),
        station::ability(
            "Station (Tap another creature you control: Put charge counters equal to its power on this Spacecraft. Station only as a sorcery. It's an artifact creature at 5+.)",
        ),
    ],
    statics: &[
        static_ability(Layer::Type, station::charged::<5>, station::is_a_creature, FIVE_PLUS),
        static_ability(Layer::Ability, statics::itself, have_flying_and_trample, FIVE_PLUS).continues(),
        static_ability(Layer::PtSet, statics::itself, statics::base_pt::<10, 10>, FIVE_PLUS).continues(),
    ],
    ..oracle::EXTINGUISHER_BATTLESHIP
};
