#[derive(Default)]
pub(crate) enum IdentifiableAbortRound {
    FirstRound,
    SecondRound,
    #[default]
    None,
}

impl IdentifiableAbortRound {
    pub(crate) fn new() {}

    pub(crate) fn complete_round() {}
}

pub(crate) enum IdentifiableAbortRoundCompletion {
    Message(),
    FirstRoundOutput(),
    SecondRoundOutput(),
    None,
}

#[derive(Clone)]
pub(crate) struct IdentifiableAbortState {
    party_id: PartyID,
    parties: HashSet<PartyID>,
}

impl IdentifiableAbortState {
    pub(crate) fn new(
        party_id: PartyID,
        parties: HashSet<PartyID>,
    ) -> Self {
        Self {
            party_id,
            parties: parties.clone(),
        }
    }
}
