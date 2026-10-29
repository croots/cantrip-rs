use cantrip_api::*;
use cantrip_mechanics::effects;
use cantrip_oracle as oracle;

/// The return works only from the graveyard (CR 113.6m), where the card is not a creature.
pub static CARD: CardDef = CardDef {
    abilities: &[ability(cost("{4}{B}"), effects::return_this_card_to_battlefield_tapped, "{4}{B}: Return this card from your graveyard to the battlefield tapped.")
        .from(ZoneKind::Graveyard)],
    ..oracle::TUNNEL_RATS
};
