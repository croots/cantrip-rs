use cantrip_api::*;
use cantrip_mechanics::{each_opponent_sacrifices, each_opponent_sacrifices_a_nontoken_creature};
use cantrip_oracle as oracle;

fn creature_token(g: &dyn Query, id: ObjectId) -> bool {
    g.chars(id).is(CardType::Creature) && g.obj(id).token
}

fn planeswalker(g: &dyn Query, id: ObjectId) -> bool {
    g.chars(id).is(CardType::Planeswalker)
}

fn sacrifice_nontoken(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> {
    each_opponent_sacrifices_a_nontoken_creature(g, item.controller, item.object)
}

fn sacrifice_token(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> {
    each_opponent_sacrifices(g, item.controller, item.object, 1, creature_token)
}

fn sacrifice_planeswalker(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> {
    each_opponent_sacrifices(g, item.controller, item.object, 1, planeswalker)
}

static MODAL: ModalDef = modes(&[
    mode(sacrifice_nontoken, "Each opponent sacrifices a nontoken creature of their choice."),
    mode(sacrifice_token, "Each opponent sacrifices a creature token of their choice."),
    mode(sacrifice_planeswalker, "Each opponent sacrifices a planeswalker of their choice."),
]);

pub static CARD: CardDef = CardDef { spell: Some(SpellDef::MODAL), modal: Some(&MODAL), ..oracle::SHEOLDREDS_EDICT };
