use crate::PublicKey;
use crate::{ByzantineRequirement, Index, Proposal};
use gwrdfa_resample::agreement::{std::join_set_committee::TakesJoinSet, Condition, Subcommittee};
use std::{
	collections::{BTreeSet, HashMap},
	hash::Hash,
};

/// Aegeri subcommittee carrying the sender membership and active stage index.
///
/// Consensus evaluation mirrors `VoterSet` sender-accounting, then delegates
/// stage-specific aggregation to `Proposal::consensus_condition_for_index`.
#[derive(Debug, Eq, PartialEq, Clone, Hash, PartialOrd, Ord)]
pub struct AegeriSubcommittee {
	// Inclduding the index in the subcommittee is not strictly necessary,
	// but it serves as a final safety measure.
	index: Index,
	members: BTreeSet<PublicKey>,
}

impl AegeriSubcommittee {
	pub fn new(index: Index) -> Self {
		Self { index, members: BTreeSet::new() }
	}

	pub fn index(&self) -> &Index {
		&self.index
	}

	pub fn senders(&self) -> impl Iterator<Item = &PublicKey> {
		self.members.iter()
	}

	pub fn with_members(mut self, members: impl Iterator<Item = PublicKey>) -> Self {
		self.members.extend(members);
		self
	}

	pub fn add_member(&mut self, member: PublicKey) {
		self.members.insert(member);
	}

	pub fn remove_member(&mut self, member: PublicKey) {
		self.members.remove(&member);
	}

	pub fn size(&self) -> usize {
		self.members.len()
	}
}

impl Subcommittee<Proposal> for AegeriSubcommittee {
	fn condition<'a>(
		&'a self,
		partials: impl Iterator<Item = (&'a Self, &'a Proposal)> + 'a,
	) -> Condition<Proposal> {
		let mut sender_to_proposal: HashMap<&PublicKey, &Proposal> = HashMap::new();
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

impl TakesJoinSet<Proposal> for AegeriSubcommittee {
	fn update_with_join_set(
		&mut self,
		joiners: impl Iterator<Item = Self>,
		leavers: impl Iterator<Item = Self>,
	) {
		for joiner in joiners {
			for member in joiner.members {
				self.add_member(member);
			}
		}
		for leaver in leavers {
			for member in leaver.members {
				self.remove_member(member);
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::super::IndexValue;
	use super::*;

	#[test]
	fn test_aegeri_subcommittee_reaches_consensus() {
		let committee = AegeriSubcommittee::new(Index::Availability(IndexValue(0))).with_members(
			vec![
				PublicKey::new_for_test(1),
				PublicKey::new_for_test(2),
				PublicKey::new_for_test(3),
			]
			.into_iter(),
		);
		let proposal = Proposal::Availability(crate::Availability::new());
		let condition = committee.condition(vec![(&committee, &proposal)].into_iter());
		assert_eq!(condition, Condition::Consensus(proposal));
	}

	#[test]
	fn test_aegeri_subcommittee_hung_on_sender_conflict() {
		let committee = AegeriSubcommittee::new(Index::Availability(IndexValue(0))).with_members(
			vec![
				PublicKey::new_for_test(1),
				PublicKey::new_for_test(2),
				PublicKey::new_for_test(3),
			]
			.into_iter(),
		);
		let left = AegeriSubcommittee::new(Index::Availability(IndexValue(0)))
			.with_members(vec![PublicKey::new_for_test(1), PublicKey::new_for_test(2)].into_iter());
		let right = AegeriSubcommittee::new(Index::Availability(IndexValue(0)))
			.with_members(vec![PublicKey::new_for_test(2), PublicKey::new_for_test(3)].into_iter());
		let availability = Proposal::Availability(crate::Availability::new());
		let confirmation = Proposal::Confirmation(crate::Confirmation::new());
		let condition =
			committee.condition(vec![(&left, &availability), (&right, &confirmation)].into_iter());
		assert_eq!(condition, Condition::Hung);
	}
}
