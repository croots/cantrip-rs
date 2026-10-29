use cantrip_api::*;
use cantrip_mechanics::mana;
use cantrip_oracle as oracle;

pub static CARD: CardDef = CardDef {
    abilities: &[triggered(
        when(EventMask::TAPPED, mana::you_tap_a_land_for_mana),
        mana::add_an_additional_green,
        "Whenever you tap a land for mana, add {G}.",
    )
    .mana_ability(TriggeredMana::One(Mana::G))],
    ..oracle::GROUNDCHUCK_DIRTBAG
};
