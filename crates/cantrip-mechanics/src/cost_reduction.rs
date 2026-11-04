//! "This spell costs {1} less to cast for each ..." where the card is not affinity (CR 702.41a is the keyword, and only the cards
//! that print the word use `affinity`): a cost reduction (CR 601.2f, 118.7) that counts a pool of the caster's.

use cantrip_api::*;

/// "This spell costs {1} less to cast for each instant and sorcery card in your graveyard" (CR 601.2f, 118.7a): counted as
/// the cost is determined, the cards as they are in the graveyard. Eddymurk Crab and The Dawning Archaic print it.
pub fn for_each_instant_and_sorcery_in_your_graveyard(q: &dyn Query, _: ObjectId, player: PlayerId, _: Costed, _: &[Target]) -> CostDelta {
    let cards = q.state().zone(Zone::graveyard(player)).iter().filter(|&&id| q.chars(id).is(CardType::Instant) || q.chars(id).is(CardType::Sorcery)).count();
    CostDelta::less(cards as u32)
}

/// "This spell costs {N} less to cast if you control a [subtype]" (CR 601.2f, 118.7a): judged once, as the total cost is
/// determined, for the cost of casting it only (the core asks a card's own change only as it is cast), and counting only the
/// caster's permanents. `TYPE` is `lands::sub(Subtype::Kithkin)` and so on; the type is a word of the spell's rules text, read
/// through its words (CR 612.2), and a permanent has it when the layers made it that type. Mistmeadow Council (Kithkin), Visions of Villainy (Villain), Saved by the
/// Shell (Turtle), Truck Toss (Vehicle, {2}) and Geistlight Snare (Spirit) print it.
pub fn less_if_you_control_subtype<const N: u8, const TYPE: u16>(q: &dyn Query, source: ObjectId, player: PlayerId, _: Costed, _: &[Target]) -> CostDelta {
    let wanted = crate::lands::word(q, source, TYPE);
    if q.permanents_of(player).into_iter().any(|id| q.chars(id).has_subtype(wanted)) { CostDelta::less(N as u32) } else { CostDelta::NONE }
}

/// "The second spell you cast each turn costs {N} less to cast" (CR 601.2f, 118.7a, 601.2i): the controller's own spells only,
/// the one being cast being their second when exactly one spell of theirs was cast earlier this turn, whoever's it was and
/// however it was cast. A generic reduction takes only the generic part (CR 118.7c). Uthros Psionicist ({2}), Highspire
/// Bell-Ringer, Monk Class and Raging Battle Mouse ({1}) print it.
pub fn second_spell_you_cast_each_turn_costs_less<const N: u8>(q: &dyn Query, source: ObjectId, player: PlayerId, costed: Costed, _: &[Target]) -> CostDelta {
    if !matches!(costed, Costed::Spell(_)) || player != q.controller(source) {
        return CostDelta::NONE;
    }
    if q.history().casts_by(player).count() == 1 { CostDelta::less(N as u32) } else { CostDelta::NONE }
}
