use cantrip_api::*;
use cantrip_mechanics::triggers::LANDFALL;
use cantrip_mechanics::{effects, tokens};
use cantrip_oracle as oracle;

/// "Create a 2/2 green Bird creature token with 'Whenever a land you control enters, this token gets +1/+0 until end of turn.'"
/// (CR 111.1, 701.7a)
fn make_a_bird(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> {
    tokens::create(g, item.controller, &BIRD, 1)?;
    Ok(())
}

/// The 2/2 green Bird creature token it makes, with its own landfall ability (CR 111.4).
static BIRD: CardDef = CardDef {
    colors: Some(ColorSet::of(Color::Green)),
    types: TypeSet::of(&[CardType::Creature]),
    power: Some(2),
    toughness: Some(2),
    abilities: &[triggered(
        LANDFALL,
        effects::this_gets::<1, 0>,
        "Whenever a land you control enters, this token gets +1/+0 until end of turn.",
    )],
    ..token!(Bird)
};

pub static CARD: CardDef = CardDef {
    abilities: &[triggered(
        LANDFALL,
        make_a_bird,
        "Landfall \u{2014} Whenever a land you control enters, create a 2/2 green Bird creature token with \"Whenever a land you control enters, this token gets +1/+0 until end of turn.\"",
    )],
    ..oracle::CHOCOBO_RACETRACK
};

pub static EXTRA: &[&CardDef] = &[&BIRD];
