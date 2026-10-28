//! Desert Were-Worm: "Whenever you attack with creatures with total power 12 or greater for the first time each turn" is about
//! the first declaration this turn that reaches 12, not the first declaration. A scenario cannot field this many cards by turn
//! seven, so the board is set up directly (as `combat_counts` does) and only the declarations are played.
//!
//! The Worm is a 8/5 with four Mountains. Declaration 1: the Worm and Aurelia, the Warleader attack for exactly 11 - no Worm
//! trigger - and Aurelia's own trigger (her first attack) gives an additional combat. Declaration 2: the Worm, Aurelia and a
//! Memnite attack for 12, the first declaration to reach it, so the Worm triggers and gives one more combat. Declaration 3: the same
//! 12 again, not the first, so there is no fourth. Three declarations in all.

use std::sync::OnceLock;

use cantrip_engine::*;

const P0: PlayerId = PlayerId(0);
const P1: PlayerId = PlayerId(1);

fn table() -> &'static CardTable {
    static TABLE: OnceLock<CardTable> = OnceLock::new();
    TABLE.get_or_init(|| CardTable::new(cantrip_cards::all()))
}

#[test]
fn an_earlier_declaration_of_eleven_does_not_count_and_one_of_twelve_does() {
    let card = |name: &str| table().find(name).unwrap_or_else(|| panic!("no card {name}"));
    let deck = || vec![card("Plains"); 30];
    let mut game = Game::new(table(), &Setup { decks: vec![deck(), deck()], seed: 1, shuffle: false, opening_chooser: None, ..Default::default() });
    game.state.update_player(P1, |p| p.life = 1000);
    let mut put = |game: &mut Game, name: &str| {
        let id = game.state.push_obj(Obj::new(card(name), P0, Zone::BATTLEFIELD));
        game.state.zone_push(Zone::BATTLEFIELD, id);
        id
    };
    let worm = put(&mut game, "Desert Were-Worm");
    for _ in 0..4 {
        put(&mut game, "Mountain");
    }
    let aurelia = put(&mut game, "Aurelia, the Warleader");
    let memnite = put(&mut game, "Memnite");
    game.state.see_statics();
    let mut declarations = 0;
    for _ in 0..2000 {
        game.run();
        let Some(request) = game.pending().cloned() else { break };
        if game.state.turn.number > 1 {
            break;
        }
        let answer = match &request.kind {
            RequestKind::DeclareAttackers { .. } if request.player == P0 => {
                declarations += 1;
                if declarations == 1 {
                    assert_eq!(game.chars(worm).power, Some(8), "two for each of four Mountains");
                }
                let attackers = if declarations == 1 { vec![worm, aurelia] } else { vec![worm, aurelia, memnite] };
                Answer::Attack(attackers.into_iter().map(|a| (a, Target::Player(P1))).collect())
            }
            RequestKind::DeclareAttackers { .. } => Answer::Attack(Vec::new()),
            RequestKind::DeclareBlockers { .. } => Answer::Block(Vec::new()),
            RequestKind::DiscardToHandSize { count, hand } => Answer::Cards(hand.iter().copied().take(*count as usize).collect()),
            _ => Answer::Pass,
        };
        game.answer(request.id, request.player, answer).unwrap_or_else(|e| panic!("{:?} refused: {e:?}", request.kind));
    }
    assert_eq!(declarations, 3, "the 11 does not trigger the Worm, the first 12 does, the second 12 does not");
}
