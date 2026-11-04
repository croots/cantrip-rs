use cantrip_api::*;
use cantrip_mechanics::attach::{attach_to_target_and_gain_until_end_of_turn, attached_object, equip};
use cantrip_mechanics::statics::plus_pt;
use cantrip_mechanics::{targets, triggers::THIS_ENTERS};
use cantrip_oracle as oracle;

/// Flash is a core keyword read from its characteristics.
pub static CARD: CardDef = CardDef {
    abilities: &[
        triggered(
            THIS_ENTERS,
            |g, item| attach_to_target_and_gain_until_end_of_turn(g, item, Keyword::Wither),
            "When this Equipment enters, attach it to target creature you control. That creature gains wither until end of turn.",
        )
        .targets(&[targets::CREATURE_YOU_CONTROL]),
        equip(mana("{2}"), "Equip {2}"),
    ],
    statics: &[static_ability(Layer::PtModify, attached_object, plus_pt::<1, 2>, "Equipped creature gets +1/+2.")],
    ..oracle::BARBED_BLOODLETTER
};
