use cantrip_api::*;
use cantrip_mechanics::tokens::{self, ROBOT_2_2};
use cantrip_mechanics::triggers::THIS_DIES;
use cantrip_oracle as oracle;

pub static CARD: CardDef = CardDef {
    abilities: &[triggered(THIS_DIES, tokens::create_a_tapped_robot, "When this creature dies, create a tapped 2/2 colorless Robot artifact creature token.")],
    ..oracle::GRAVPACK_MONOIST
};

pub static EXTRA: &[&CardDef] = &[&ROBOT_2_2];
