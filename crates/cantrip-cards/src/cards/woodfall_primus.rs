use cantrip_api::*;
use cantrip_mechanics::{destroy_target, persist, targets};
use cantrip_mechanics::triggers::THIS_ENTERS;
use cantrip_oracle as oracle;

fn destroy_it(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> {
    destroy_target(g, item, 0);
    Ok(())
}

pub static CARD: CardDef = CardDef {
    abilities: &[
        triggered(THIS_ENTERS, destroy_it, "When this creature enters, destroy target noncreature permanent.")
            .targets(&[targets::NONCREATURE_PERMANENT]),
        persist::trigger("Persist (When this creature dies, if it had no -1/-1 counters on it, return it to the battlefield under its owner's control with a -1/-1 counter on it.)"),
    ],
    ..oracle::WOODFALL_PRIMUS
};
