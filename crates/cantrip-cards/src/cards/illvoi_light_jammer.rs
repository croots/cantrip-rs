use cantrip_api::*;
use cantrip_mechanics::attach::{attach_to_target_which_gains_hexproof, attached_object, equip};
use cantrip_mechanics::statics::plus_pt;
use cantrip_mechanics::{targets, triggers::THIS_ENTERS};
use cantrip_oracle as oracle;

/// Flash is a core keyword read from its characteristics.
pub static CARD: CardDef = CardDef {
    abilities: &[
        triggered(
            THIS_ENTERS,
            attach_to_target_which_gains_hexproof,
            "When this Equipment enters, attach it to target creature you control. That creature gains hexproof until end of turn.",
        )
        .targets(&[targets::CREATURE_YOU_CONTROL]),
        equip(mana("{3}"), "Equip {3}"),
    ],
    statics: &[static_ability(Layer::PtModify, attached_object, plus_pt::<1, 2>, "Equipped creature gets +1/+2.")],
    ..oracle::ILLVOI_LIGHT_JAMMER
};
