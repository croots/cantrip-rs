use cantrip_api::*;
use cantrip_mechanics::effects::return_first_target_to_hand;
use cantrip_mechanics::{targets, triggers::THIS_ENTERS};
use cantrip_oracle as oracle;

/// "Target Villain card from your graveyard" (CR 115.1, 205.3), read through the words of this creature's text (CR 612): a card in its
/// controller's graveyard that is a Villain there.
fn villain_card_in_your_graveyard(q: &dyn Query, asker: &Asker, t: Target) -> bool {
    targets::card_in_your_graveyard(q, asker, t, &[])
        && matches!(t, Target::Object(id) if q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Villain)))
}

/// "Villain spells you cast cost {1} less to cast" (CR 601.2f): a spell of yours that is a Villain as it is cast, read through the
/// words of this creature's text (CR 612); generic mana only (CR 118.7a).
fn villain_spells_cost_one_less(q: &dyn Query, source: ObjectId, player: PlayerId, costed: Costed, _: &[Target]) -> CostDelta {
    let Costed::Spell(spell) = costed else { return CostDelta::NONE };
    if player == q.controller(source) && q.chars(spell).has_subtype(q.words(source).subtype(Subtype::Villain)) {
        CostDelta::less(1)
    } else {
        CostDelta::NONE
    }
}

pub static CARD: CardDef = CardDef {
    abilities: &[triggered(THIS_ENTERS, return_first_target_to_hand, "When Tombstone enters, return target Villain card from your graveyard to your hand.")
        .targets(&[target(villain_card_in_your_graveyard)])],
    cost_changes: &[villain_spells_cost_one_less],
    ..oracle::TOMBSTONE_CAREER_CRIMINAL
};
