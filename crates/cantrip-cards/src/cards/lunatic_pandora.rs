use cantrip_api::*;
use cantrip_mechanics::{destroy_target, effects, targets};
use cantrip_oracle as oracle;

pub static CARD: CardDef = CardDef {
    abilities: &[
        ability(cost("{2}").and(TAP), |g, item| effects::surveil(g, item, 1), "{2}, {T}: Surveil 1."),
        ability(
            cost("{6}").and(TAP).and(SACRIFICE_THIS),
            |g, item| {
                destroy_target(g, item, 0);
                Ok(())
            },
            "{6}, {T}, Sacrifice this artifact: Destroy target nonland permanent.",
        )
        .targets(&[targets::NONLAND_PERMANENT]),
    ],
    ..oracle::LUNATIC_PANDORA
};
