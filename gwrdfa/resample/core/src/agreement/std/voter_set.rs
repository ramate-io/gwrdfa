use crate::agreement::{Condition, Subcommittee};
use std::{
	collections::{BTreeSet, HashMap},
	hash::Hash,
	vec::Vec,
};

/// A concrete `Subcommittee` implementation represented as a set of voters.
///
/// Each partial contributes `(voter-set, value)` evidence. The condition logic:
/// - returns `Consensus(v)` when at least `2f + 1` voters support `v`,
/// - returns `Hung` when remaining unknown votes can no longer satisfy quorum,
/// - otherwise returns `InProgress`.
#[derive(Debug, Eq, PartialEq, Clone, Hash, PartialOrd, Ord, Default)]
pub struct VoterSet<Sender: PartialEq + Eq + PartialOrd + Ord + Hash + Clone> {
	members: BTreeSet<Sender>,
}

impl<Sender: PartialEq + Eq + PartialOrd + Ord + Hash + Clone> VoterSet<Sender> {
	pub fn new() -> Self {
		Self { members: BTreeSet::new() }
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

	pub fn byzantine_quorum_size(&self) -> usize {
		(self.size() * 2).div_ceil(3) + 1
	}
}

impl<
		Sender: PartialEq + Eq + PartialOrd + Ord + Hash + Clone,
		Value: PartialEq + Eq + Hash + Clone + 'static,
	> Subcommittee<Value> for VoterSet<Sender>
{
	fn condition<'a>(
		&'a self,
		partials: impl Iterator<Item = (&'a Self, &'a Value)> + 'a,
	) -> Condition<Value> {
		let mut sender_to_values: HashMap<&Sender, &Value> = HashMap::new();
		for (subcommittee, value) in partials {
			for sender in subcommittee.senders() {
				match sender_to_values.get(sender) {
					Some(existing) if *existing != value => return Condition::Hung,
					Some(_) => {}
					None => {
						sender_to_values.insert(sender, value);
					}
				}
			}
		}
		let senders_count = sender_to_values.len();

		let mut values_to_senders: HashMap<&Value, Vec<&Sender>> = HashMap::new();
		for (sender, value) in sender_to_values {
			values_to_senders.entry(value).or_default().push(sender);
		}

		let mut max_senders = 0;
		let byzantine_quorum_size = self.byzantine_quorum_size();
		for (value, senders) in values_to_senders {
			max_senders = max_senders.max(senders.len());
			if senders.len() >= byzantine_quorum_size {
				return Condition::Consensus(value.clone());
			}
		}

		if (self.size() - senders_count) < (byzantine_quorum_size - max_senders) {
			return Condition::Hung;
		}

		Condition::InProgress
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn voter_set_reaches_consensus() {
		let mut voters = VoterSet::<u32>::new();
		voters.add_member(1);
		voters.add_member(2);
		voters.add_member(3);
		voters.add_member(4);
		voters.add_member(5);
		voters.add_member(6);
		assert_eq!(voters.condition(vec![(&voters, &1)].into_iter()), Condition::Consensus(1));
	}
}
