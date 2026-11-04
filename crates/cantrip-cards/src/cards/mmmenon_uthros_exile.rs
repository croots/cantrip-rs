use cantrip_api::*;
use cantrip_mechanics::{effects, targets};
use cantrip_oracle as oracle;

/// "Whenever an artifact you control enters" (CR 603.6a): an artifact, token or card, under your control as it entered. The
/// source is not itself an artifact, so it never answers its own entering.
fn artifact_you_control_enters(q: &dyn Query, me: &Asker, event: &Event) -> Option<Fired> {
    match event {
        Event::ZoneChange { new, to, .. }
            if to.is(ZoneKind::Battlefield) && q.controller(*new) == me.controller && q.chars(*new).is(CardType::Artifact) =>
        {
            Fired::PLAIN
        }
        _ => None,
    }
}

/// Flying is in the content JSON.
pub static CARD: CardDef = CardDef {
    abilities: &[triggered(
        when_asked(EventMask::ZONE_CHANGE, artifact_you_control_enters),
        effects::plus_one_counter_on_target,
        "Whenever an artifact you control enters, put a +1/+1 counter on target creature.",
    )
    .targets(&[targets::CREATURE])],
    ..oracle::MMMENON_UTHROS_EXILE
};
