use crate::{ByzantineRequirement, Index, Proposal};
use gwrdfa_resample::agreement::{Condition, Subcommittee};
use std::{
	collections::{BTreeSet, HashMap},
	hash::Hash,
};

/// Aegeri subcommittee carrying the sender membership and active stage index.
///
/// Consensus evaluation mirrors `VoterSet` sender-accounting, then delegates
/// stage-specific aggregation to `Proposal::consensus_condition_for_index`.
#[derive(Debug, Eq, PartialEq, Clone, Hash, PartialOrd, Ord)]
pub struct AegeriSubcommittee<Sender: PartialEq + Eq + PartialOrd + Ord + Hash + Clone> {
	index: Index,
	members: BTreeSet<Sender>,
}

impl<Sender: PartialEq + Eq + PartialOrd + Ord + Hash + Clone> AegeriSubcommittee<Sender> {
	pub fn new(index: Index) -> Self {
		Self { index, members: BTreeSet::new() }
	}

	pub fn index(&self) -> &Index {
		&self.index
	}

	pub fn senders(&self) -> impl Iterator<Item = &Sender> {
		self.members.iter()
	}

	pub fn with_members(mut self, members: impl Iterator<Item = Sender>) -> Self {
		self.members.extend(members);
		self
	}

	pub fn add_member(&mut self, member: Sender) {
		self.members.insert(member);
	}

	pub fn size(&self) -> usize {
		self.members.len()
	}
}

impl<Sender: PartialEq + Eq + PartialOrd + Ord + Hash + Clone> Subcommittee<Proposal>
	for AegeriSubcommittee<Sender>
{
	fn condition<'a>(
		&'a self,
		partials: impl Iterator<Item = (&'a Self, &'a Proposal)> + 'a,
	) -> Condition<Proposal> {
		let mut sender_to_proposal: HashMap<&Sender, &Proposal> = HashMap::new();
		for (subcommittee, proposal) in partials {
			// Ignore stale/future stage subcommittees and only evaluate active index.
			if subcommittee.index() != self.index() {
				continue;
			}
			for sender in subcommittee.senders() {
				match sender_to_proposal.get(sender) {
					Some(existing) if *existing != proposal => return Condition::Hung,
					Some(_) => {}
					None => {
						sender_to_proposal.insert(sender, proposal);
					}
				}
			}
		}

		let requirement = ByzantineRequirement::byzantine_quorum(self.size());
		Proposal::consensus_condition_for_index(
			self.index(),
			sender_to_proposal.into_values(),
			requirement,
		)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_aegeri_subcommittee_reaches_consensus() {
		let committee = AegeriSubcommittee::<u32>::new(Index::Availability(0))
			.with_members(vec![1, 2, 3].into_iter());
		let proposal = Proposal::Availability(crate::Availability::new());
		let condition = committee.condition(vec![(&committee, &proposal)].into_iter());
		assert_eq!(condition, Condition::Consensus(proposal));
	}

	#[test]
	fn test_aegeri_subcommittee_hung_on_sender_conflict() {
		let committee = AegeriSubcommittee::<u32>::new(Index::Availability(0))
			.with_members(vec![1, 2, 3].into_iter());
		let left = AegeriSubcommittee::<u32>::new(Index::Availability(0))
			.with_members(vec![1, 2].into_iter());
		let right = AegeriSubcommittee::<u32>::new(Index::Availability(0))
			.with_members(vec![2, 3].into_iter());
		let availability = Proposal::Availability(crate::Availability::new());
		let confirmation = Proposal::Confirmation(crate::Confirmation::new());
		let condition = committee.condition(
			vec![(&left, &availability), (&right, &confirmation)].into_iter(),
		);
		assert_eq!(condition, Condition::Hung);
	}
}
