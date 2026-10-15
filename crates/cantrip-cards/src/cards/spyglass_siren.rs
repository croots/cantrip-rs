use cantrip_api::*;
use cantrip_mechanics::tokens::create_map;
use cantrip_mechanics::triggers::this_enters;
use cantrip_oracle as oracle;

fn map(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> {
    create_map(g, item.controller)
}

pub static CARD: CardDef = CardDef {
    abilities: &[AbilityDef::Triggered(TriggeredDef {
        text: "When this creature enters, create a Map token.",
        listens: EventMask::ZONE_CHANGE,
        matches: this_enters,
        effect: map,
        ..TriggeredDef::BLANK
    })],
    ..oracle::SPYGLASS_SIREN
};
