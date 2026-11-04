use cantrip_api::*;
use cantrip_mechanics::{effects, pump};
use cantrip_oracle as oracle;

/// "This creature gets +1/+0 and gains menace until end of turn" (CR 611.2, 702.110): two continuous effects from the resolving
/// ability's source, nothing if the creature has left the battlefield.
fn pump_and_menace(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> {
    pump(g, item.object, item.object, 1, 0);
    if g.is_on_battlefield(item.object) {
        effects::gain_until_end_of_turn(g, item.object, item.object, Keyword::Menace);
    }
    Ok(())
}

pub static CARD: CardDef = CardDef {
    abilities: &[ability(
        discard_cards(1),
        pump_and_menace,
        "Discard a card: This creature gets +1/+0 and gains menace until end of turn. Activate only as a sorcery.",
    )
    .sorcery_speed()],
    ..oracle::INNER_DEMONS_GANGSTERS
};
