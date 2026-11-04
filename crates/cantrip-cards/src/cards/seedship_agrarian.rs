use cantrip_api::*;
use cantrip_mechanics::triggers::{LANDFALL, THIS_BECOMES_TAPPED};
use cantrip_mechanics::{effects, tokens};
use cantrip_oracle as oracle;

pub static CARD: CardDef = CardDef {
    abilities: &[
        triggered(THIS_BECOMES_TAPPED, tokens::create_a_lander, "Whenever this creature becomes tapped, create a Lander token."),
        triggered(
            LANDFALL,
            effects::plus_one_counter_on_this,
            "Landfall \u{2014} Whenever a land you control enters, put a +1/+1 counter on this creature.",
        ),
    ],
    ..oracle::SEEDSHIP_AGRARIAN
};

pub static EXTRA: &[&CardDef] = &[&tokens::LANDER];
