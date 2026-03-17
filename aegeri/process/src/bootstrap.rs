use crate::AegeriParabyzantineData;
use aegeri_message::{
	AegeriSubcommittee, Index as AegeriIndex, Proposal as AegeriProposal, PublicKey,
};
use gwrdfa_container::query::matching_tuple::MatchingTuple;
use gwrdfa_resample::{
	agreement::std::{Index as ResampleIndex, Subcom, Value as ResampleValue},
	ForResample,
};
use parabyzantine::agreement::{Agreement, AgreementWorld, ParabyzantineAgreement};
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Bootstrap {
	bootstrapped: bool,
	peer_count_required: usize,
	bootstrap_peers: HashSet<PublicKey>,
	counts: HashMap<(AegeriIndex, AegeriSubcommittee), HashSet<PublicKey>>,
}

impl Bootstrap {
	pub fn new() -> Self {
		// By default, we assume the node has already bootstrapped.
		Self {
			bootstrapped: true,
			peer_count_required: 0,
			bootstrap_peers: HashSet::new(),
			counts: HashMap::new(),
		}
	}

	pub fn with_bootstrapped(mut self, has_bootstrapped: bool) -> Self {
		self.bootstrapped = has_bootstrapped;
		self
	}

	pub fn has_bootstrapped(&self) -> bool {
		self.bootstrapped
	}

	pub fn contains_peer(&self, peer: &PublicKey) -> bool {
		self.bootstrap_peers.contains(peer)
	}

	pub fn add_peer(&mut self, peer: PublicKey) {
		self.bootstrap_peers.insert(peer);
	}

	pub fn remove_peer(&mut self, peer: PublicKey) {
		self.bootstrap_peers.remove(&peer);
	}

	pub fn with_bootstrap_peers(
		mut self,
		bootstrap_peers: impl IntoIterator<Item = PublicKey>,
	) -> Self {
		self.bootstrap_peers.extend(bootstrap_peers);
		self
	}
}

impl ParabyzantineAgreement<AegeriParabyzantineData> for Bootstrap {
	fn update_parabyzantine_agreement(
		&mut self,
		world: &mut AgreementWorld<AegeriParabyzantineData>,
	) {
		for (entity, (_marker, index, proposal, sending_subcommittee)) in
			world.certificate_facts.query(MatchingTuple::<(
				ForResample,
				ResampleIndex<AegeriIndex>,
				ResampleValue<AegeriProposal>,
				Subcom<AegeriSubcommittee>,
			)>::new())
		{
			match &proposal.0 {
				AegeriProposal::SubcommitteeBroadcast(subcommittee) => {
					for peer in sending_subcommittee.0.senders() {
						self.counts
							.entry((index.0, subcommittee.clone()))
							.or_insert(HashSet::new())
							.insert(peer.clone());
					}

					if !self.bootstrapped
						&& self
							.counts
							.get(&(index.0, subcommittee.clone()))
							.unwrap_or(&HashSet::new())
							.len() >= self.peer_count_required
					{
						world.agreement_inferences.insert(
							None,
							(
								Agreement,
								ResampleIndex::new(index.0),
								Subcom::new(subcommittee.clone()),
							),
						);
						// Technically, this would be safer to
						// insert as an inference
						// or to check the length of the agreements buffer.
						// But, the current implementation is for simplicity.
						self.bootstrapped = true;
					}

					// We can remove the certificate inference.
					// This is a consuming stage in the parabyzantine tick.
					world.certificate_inferences.remove_entity(entity);
				}
				_ => {}
			}
		}
	}
}
