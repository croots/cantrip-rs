use cantrip_api::*;
use cantrip_mechanics::{hand, statics, station, triggers::THIS_ATTACKS};
use cantrip_oracle as oracle;

const SEVEN_PLUS: &str = "7+ | Flying";

/// "You may put a creature card from your hand onto the battlefield" (CR 701.20, 608.2c): nobody is asked with no creature card
/// in hand, and only cards in the controller's own hand count.
fn put_a_creature_card(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> {
    hand::may_put_onto_battlefield(g, item.controller, item.object, |c| c.is(CardType::Creature))?;
    Ok(())
}

pub static CARD: CardDef = CardDef {
    abilities: &[
        station::ability("Station (Tap another creature you control: Put charge counters equal to its power on this Spacecraft. Station only as a sorcery. It's an artifact creature at 7+.)"),
        triggered(THIS_ATTACKS, put_a_creature_card, "Whenever this Spacecraft attacks, you may put a creature card from your hand onto the battlefield."),
    ],
    statics: &[
        static_ability(Layer::Type, station::charged::<7>, station::is_a_creature, SEVEN_PLUS),
        static_ability(Layer::Ability, statics::itself, statics::have_flying, SEVEN_PLUS).continues(),
        static_ability(Layer::PtSet, statics::itself, statics::base_pt::<4, 5>, SEVEN_PLUS).continues(),
    ],
    ..oracle::SLEDGE_CLASS_SEEDSHIP
};
