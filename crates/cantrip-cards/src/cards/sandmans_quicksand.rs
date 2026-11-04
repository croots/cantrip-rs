use cantrip_api::*;
use cantrip_mechanics::{mayhem, pump};
use cantrip_oracle as oracle;

/// "All creatures get -2/-2 until end of turn. If this spell's mayhem cost was paid, creatures your opponents control get
/// -2/-2 until end of turn instead." The creatures on the battlefield as the spell resolves (CR 611.2c), those an opponent
/// controls when the mayhem way was used (CR 702.187a).
fn effect(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> {
    let mayhem_paid = item.way == 1;
    let creatures: Vec<ObjectId> = g
        .state()
        .battlefield()
        .iter()
        .copied()
        .filter(|&id| g.chars(id).is(CardType::Creature) && (!mayhem_paid || g.controller(id) != item.controller))
        .collect();
    instruction(g, |g| {
        for id in creatures {
            pump(g, item.object, id, -2, -2);
        }
    });
    Ok(())
}

pub static CARD: CardDef = CardDef {
    spell: spell(&[], effect),
    ways: &[mayhem::way(mana("{3}{B}"), "Mayhem {3}{B}")],
    ..oracle::SANDMANS_QUICKSAND
};
