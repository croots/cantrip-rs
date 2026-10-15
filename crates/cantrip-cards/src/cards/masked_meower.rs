use cantrip_api::*;
use cantrip_mechanics::effects;
use cantrip_mechanics::targets::filters;
use cantrip_oracle as oracle;

pub static CARD: CardDef = CardDef {
    abilities: &[AbilityDef::Activated(ActivatedDef {
        text: "Discard a card, Sacrifice this creature: Draw a card.",
        // Both are costs (CR 602.2b, 601.2h), paid as it is activated: a creature with no card in its controller's hand
        // cannot activate it, and it has no {T}, so summoning sickness does not matter (CR 302.6).
        cost: Cost { discard: Some((1, filters::any)), sacrifice_self: true, ..Cost::NONE },
        effect: effects::draw::<1>,
        ..ActivatedDef::BLANK
    })],
    ..oracle::MASKED_MEOWER
};
