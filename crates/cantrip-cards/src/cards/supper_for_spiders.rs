use cantrip_api::*;
use cantrip_mechanics::{effects, history};
use cantrip_oracle as oracle;

/// "Put onto the battlefield under your control all creature cards in your opponents' graveyards that were put there from
/// the battlefield this turn. They are Food artifacts with '{2}, {T}, Sacrifice this artifact: You gain 3 life.' (They lose
/// all other types and subtypes.)" (CR 400.7, 611.2c, 613.1d, 613.1f).
///
/// The cards are the creature cards still in an opponent's graveyard (a token is never a card there, and one that left and
/// came back is a new object) whose move into that graveyard, from the battlefield, was this turn's. They enter together under
/// the spell's controller; then, as a second step of the text, the lasting effect makes each of them only an artifact, its
/// subtypes only Food (supertypes stay, so a legendary creature is a legendary Food), with the ability. It has no end, and it
/// is about those objects alone (CR 400.7).
fn put_onto_the_battlefield_as_food(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> {
    let you = item.controller;
    let opponents: Vec<PlayerId> = g.state().opponents(you).collect();
    let mut cards = Vec::new();
    for opponent in opponents {
        for card in history::put_into_graveyard_from_battlefield_this_turn(g, opponent) {
            if g.chars(card).is(CardType::Creature) {
                cards.push((card, you, false));
            }
        }
    }
    let entered = g.put_all_onto_battlefield_as(&cards)?;
    g.end_sentence();
    let this = g.card_id(&CARD);
    let food = g.words(item.object).subtype(Subtype::Food);
    let forever = Duration::Indefinite;
    instruction(g, |g| {
        for permanent in entered {
            g.add_effect(item.object, permanent, EffectKind::SetTypes(TypeSet::of(&[CardType::Artifact])), forever);
            g.add_effect(item.object, permanent, EffectKind::LoseSubtypesOf(CardType::Artifact), forever);
            g.add_effect(item.object, permanent, EffectKind::AddSubtype(food), forever);
            g.add_effect(item.object, permanent, EffectKind::AddAbility(AbilityRef::delayed(this, 0)), forever);
        }
    });
    Ok(())
}

pub static CARD: CardDef = CardDef {
    spell: spell(&[], put_onto_the_battlefield_as_food),
    delayed: &[ability(cost("{2}, {T}").and(SACRIFICE_THIS), |g, item| effects::gain_life(g, item, 3), "{2}, {T}, Sacrifice this artifact: You gain 3 life.")],
    ..oracle::SUPPER_FOR_SPIDERS
};
