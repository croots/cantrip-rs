use cantrip_api::*;
use cantrip_mechanics::job_select::HERO;
use cantrip_mechanics::mana::enters_tapped;
use cantrip_mechanics::tokens;
use cantrip_oracle as oracle;

/// "Create a 1/1 colorless Hero creature token. Put a +1/+1 counter on it for each land you control" (CR 111.1, 122.1): the lands
/// counted as the spell resolves, after the token has entered (it is no land), as the layers have made them (a count of none puts no counter, CR 122.1).
fn create_a_hero_with_a_counter_per_land(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> {
    let you = item.controller;
    let made = tokens::create_by_rule(g, you, &HERO, 1, false)?;
    let lands = g.permanents_of(you).into_iter().filter(|&id| g.chars(id).is(CardType::Land)).count();
    if let Some(&token) = made.first()
        && is_live(g, token)
        && g.is_on_battlefield(token)
    {
        g.add_counters(token, CounterKind::PlusOne, lands as i16);
    }
    Ok(())
}

/// The Adventure (CR 715): Create a 1/1 colorless Hero creature token. Put a +1/+1 counter on it for each land you control.
pub static LASTING_FAYTH: CardDef = CardDef { spell: spell(&[], create_a_hero_with_a_counter_per_land), ..oracle::LASTING_FAYTH };

pub static CARD: CardDef = CardDef {
    enters: Some(enters_tapped),
    abilities: &[mana_ability(TAP, add(Mana::G), "{T}: Add {G}.")],
    adventure: Some(&LASTING_FAYTH),
    ..oracle::ZANARKAND_ANCIENT_METROPOLIS
};

pub static EXTRA: &[&CardDef] = &[&LASTING_FAYTH, &HERO];
