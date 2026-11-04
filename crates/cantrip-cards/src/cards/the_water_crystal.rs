use cantrip_api::*;
use cantrip_mechanics::replacements::an_opponent_would_mill;
use cantrip_oracle as oracle;

/// "Blue spells you cast cost {1} less to cast" (CR 601.2f): asked of the spell's own characteristics as it is cast; only its
/// controller's spells, and a generic reduction takes only the generic part (CR 118.7c). "Blue" is a word of the Crystal's
/// text (CR 612).
fn blue_spells_cost_less(q: &dyn Query, source: ObjectId, player: PlayerId, costed: Costed, _: &[Target]) -> CostDelta {
    let Costed::Spell(spell) = costed else { return CostDelta::NONE };
    if player != q.controller(source) {
        return CostDelta::NONE;
    }
    if q.chars(spell).colors.has(q.words(source).color(Color::Blue)) { CostDelta::less(1) } else { CostDelta::NONE }
}

/// "... they mill that many cards plus four instead" (CR 614.1a). Each such effect applies once (CR 614.5), so a second
/// Crystal adds four more.
fn that_many_plus_four(_: &dyn Query, _: &Asker, proposal: &mut Proposal) {
    if let Proposal::Mill(m) = proposal {
        m.count = m.count.saturating_add(4);
    }
}

/// "{4}{U}{U}, {T}: Each opponent mills cards equal to the number of cards in your hand." The count is read once, as the
/// ability resolves, and every opponent mills at once (CR 701.17a, 101.4).
fn opponents_mill_your_hand_size(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> {
    let count = g.state().zone(Zone::hand(item.controller)).len() as u32;
    let mills: Vec<(PlayerId, u32)> = g.state().opponents(item.controller).map(|o| (o, count)).collect();
    g.mill_together(&mills);
    Ok(())
}

pub static CARD: CardDef = CardDef {
    cost_changes: &[blue_spells_cost_less],
    replacements: &[replacement(ReplacementKind::Mill, an_opponent_would_mill, that_many_plus_four, "mills that many cards plus four instead")],
    abilities: &[ability(
        cost("{4}{U}{U}, {T}"),
        opponents_mill_your_hand_size,
        "{4}{U}{U}, {T}: Each opponent mills cards equal to the number of cards in your hand.",
    )],
    ..oracle::THE_WATER_CRYSTAL
};
