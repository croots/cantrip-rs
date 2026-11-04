use cantrip_api::*;
use cantrip_mechanics::effects;
use cantrip_oracle as oracle;

pub static CARD: CardDef = CardDef { spell: spell(&[], effects::draw_cards_and_lose_life::<2, 2>), ..oracle::NIGHTS_WHISPER };
