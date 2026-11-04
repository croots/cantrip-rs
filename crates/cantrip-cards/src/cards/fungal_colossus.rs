use cantrip_api::*;
use cantrip_mechanics::lands;
use cantrip_oracle as oracle;

pub static CARD: CardDef = CardDef {
    own_cost_changes: &[lands::spell_costs_x_less_where_x_is_differently_named_lands],
    ..oracle::FUNGAL_COLOSSUS
};
