use cantrip_api::*;
use cantrip_mechanics::tokens;
use cantrip_mechanics::triggers::LANDFALL;
use cantrip_oracle as oracle;

/// "Create a 2/2 green Bird creature token with 'Whenever a land you control enters, this token gets +1/+0 until end of turn.'"
/// (CR 111.1, 701.7a)
fn make_a_bird(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> {
    tokens::create_landfall_bird(g, item.controller)
}

pub static CARD: CardDef = CardDef {
    abilities: &[triggered(
        LANDFALL,
        make_a_bird,
        "Landfall \u{2014} Whenever a land you control enters, create a 2/2 green Bird creature token with \"Whenever a land you control enters, this token gets +1/+0 until end of turn.\"",
    )],
    ..oracle::CHOCOBO_RACETRACK
};

pub static EXTRA: &[&CardDef] = &[&tokens::LANDFALL_BIRD];
