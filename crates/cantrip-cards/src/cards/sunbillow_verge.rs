use cantrip_api::*;
use cantrip_mechanics::{lands};
use cantrip_oracle as oracle;

pub static CARD: CardDef = CardDef {
    abilities: &lands::verge::<{ lands::W }, { lands::R }, { lands::sub(Subtype::Mountain) }, { lands::sub(Subtype::Plains) }>(
        "{T}: Add {W}.",
        "{T}: Add {R}. Activate only if you control a Mountain or a Plains.",
    ),
    ..oracle::SUNBILLOW_VERGE
};
