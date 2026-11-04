use cantrip_api::select::{part, Choice, SelectFn, Verdict};
use cantrip_api::*;
use cantrip_mechanics::{channel, library, select};
use cantrip_oracle as oracle;

fn creature_or_planeswalker(q: &dyn Query, choice: &Choice, _: u32) -> Verdict {
    Verdict::each(choice, |card| q.chars(card).is(CardType::Creature) || q.chars(card).is(CardType::Planeswalker))
}

const CREATURE_OR_PLANESWALKER_CARD: select::Selection =
    select::Selection::of(select::as_many_as_can(1), &[part(&(creature_or_planeswalker as SelectFn), 0, "")]).called("creature or planeswalker");

/// "Mill three cards, then return a creature or planeswalker card from your graveyard to your hand": chosen from the
/// whole graveyard as it stands after the mill, and not optional - one is returned whenever there is one.
fn mill_then_return(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> {
    library::mill_then_return_a_card(g, item, 3, CREATURE_OR_PLANESWALKER_CARD)
}

pub static CARD: CardDef = CardDef {
    abilities: &[
        mana_ability(TAP, add(Mana::B), "{T}: Add {B}."),
        ability(cost("{3}{B}").and(DISCARD_THIS), mill_then_return, "Channel \u{2014} {3}{B}, Discard this card: Mill three cards, then return a creature or planeswalker card from your graveyard to your hand. This ability costs {1} less to activate for each legendary creature you control.")
            .from(ZoneKind::Hand)
            .cost_change(channel::cheaper_per_legendary_creature),
    ],
    ..oracle::TAKENUMA_ABANDONED_MIRE
};
