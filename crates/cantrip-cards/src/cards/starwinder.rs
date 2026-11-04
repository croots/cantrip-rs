use cantrip_api::*;
use cantrip_mechanics::triggers::creature_you_control_deals_combat_damage_to_player_for_amount;
use cantrip_mechanics::{effects, warp};
use cantrip_oracle as oracle;

/// "Whenever a creature you control deals combat damage to a player, you may draw that many cards" (CR 510.2, 603.2): one
/// trigger for each creature, drawing as many as that creature dealt. Warp {2}{U}{U} (CR 702.185a).
pub static CARD: CardDef = CardDef {
    abilities: &[triggered(
        when_asked(EventMask::DAMAGE, creature_you_control_deals_combat_damage_to_player_for_amount),
        effects::may_draw_that_many,
        "Whenever a creature you control deals combat damage to a player, you may draw that many cards.",
    )],
    ways: &[warp::way(mana("{2}{U}{U}"), "Warp {2}{U}{U}")],
    end_step_riders: warp::RIDERS,
    delayed: &[warp::EXILE],
    ..oracle::STARWINDER
};
