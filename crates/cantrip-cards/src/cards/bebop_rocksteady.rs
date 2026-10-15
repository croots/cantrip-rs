use cantrip_api::*;
use cantrip_oracle as oracle;

/// "Whenever Bebop & Rocksteady attack or block" (CR 508.1m, 509.1i): its own attack or its own block.
fn attack_or_block(_: &dyn Query, source: ObjectId, event: &Event) -> Option<Fired> {
    match event {
        Event::Attacks { attacker, .. } if *attacker == source => Fired::PLAIN,
        Event::Blocks { blocker, .. } if *blocker == source => Fired::PLAIN,
        _ => None,
    }
}

/// "Sacrifice a permanent unless you discard a card" (CR 118.12, 701.9a, 701.21a): the controller chooses as it
/// resolves. With no card in hand the discard cannot be paid and is not offered; a declined or impossible discard means
/// a permanent of their choice is sacrificed, Bebop & Rocksteady itself among the candidates.
fn sacrifice_unless_discard(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> {
    let player = item.controller;
    if !g.state().zone(Zone::hand(player)).is_empty()
        && matches!(g.ask(player, RequestKind::May { source: item.object, purpose: Purpose::Discard })?, Answer::Yes)
    {
        cantrip_mechanics::discard_chosen(g, player, item.object, 1)?;
        return Ok(());
    }
    cantrip_mechanics::sacrifice_chosen(g, player, item.object, 1, |_| true)
}

pub static CARD: CardDef = CardDef {
    abilities: &[AbilityDef::Triggered(TriggeredDef {
        text: "Whenever Bebop & Rocksteady attack or block, sacrifice a permanent unless you discard a card.",
        listens: EventMask(EventMask::ATTACKS.0 | EventMask::BLOCKS.0),
        matches: attack_or_block,
        effect: sacrifice_unless_discard,
        ..TriggeredDef::BLANK
    })],
    ..oracle::BEBOP_ROCKSTEADY
};
