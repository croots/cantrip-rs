use cantrip_api::*;
use cantrip_mechanics::{effects, lands, tokens};
use cantrip_oracle as oracle;

/// Create the two Romulans, then draw two cards and discard one (CR 111.1, 121.1, 701.9).
fn effect(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> {
    tokens::create(g, item.controller, &tokens::ROMULAN, 2)?;
    g.end_sentence();
    effects::draw_two_then_discard(g, item)
}

pub static CARD: CardDef = CardDef {
    own_cost_changes: &[lands::spell_costs_x_less_where_x_is_differently_named_lands],
    spell: spell(&[], effect),
    ..oracle::CLOAK_AND_PHASER
};

pub static EXTRA: &[&CardDef] = &[&tokens::ROMULAN];
