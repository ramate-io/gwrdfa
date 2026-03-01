use super::Condition;
use parabyzantine::NoOp;

pub trait Subcommittee<Value: Eq + 'static>: Eq {
	fn condition<'a>(
		&'a self,
		partials: impl Iterator<Item = (&'a Self, &'a Value)> + 'a,
	) -> Condition<Value>;
}

/// A [Subcommittee] for the [NoOp] struct.
impl<T: Eq + 'static> Subcommittee<T> for NoOp {
	fn condition<'a>(
		&'a self,
		_partials: impl Iterator<Item = (&'a NoOp, &'a T)> + 'a,
	) -> Condition<T> {
		Condition::InProgress
	}
}

pub trait IndexSubcommitteeAgreement<Index: Eq, Sub: Subcommittee<Value>, Value: Eq + 'static>:
	Eq
{
	/// The index of the agreement.
	fn index(&self) -> Index;

	/// The subcommittee of the agreement.
	fn subcommittee(&self) -> Sub;
}

/// A [IndexSubcommitteeAgreement] for the [NoOp] struct.
impl<T: Eq + 'static> IndexSubcommitteeAgreement<NoOp, NoOp, T> for NoOp {
	fn index(&self) -> NoOp {
		NoOp
	}
	fn subcommittee(&self) -> NoOp {
		NoOp
	}
}

#[cfg(test)]
pub mod test {

	use super::*;
	use std::{
		collections::{HashMap, HashSet},
		hash::Hash,
		vec,
		vec::Vec,
	};

	#[derive(Eq, PartialEq, Clone)]
	pub struct TestSubcommittee<Sender: PartialEq + Eq + Hash + Clone> {
		members: HashSet<Sender>,
	}

	impl<Sender: PartialEq + Eq + Hash + Clone> TestSubcommittee<Sender> {
		pub fn new() -> Self {
			Self { members: HashSet::new() }
		}

		pub fn senders(&self) -> impl Iterator<Item = &Sender> {
			self.members.iter()
		}

		pub fn add_member(&mut self, member: Sender) {
			self.members.insert(member);
		}

		pub fn union(mut self, other: Self) -> Self {
			self.members.extend(other.members);
			self
		}

		pub fn size(&self) -> usize {
			self.members.len()
		}

		pub fn byzantine_quorum_size(&self) -> usize {
			(self.size() + 2) / 3
		}

		pub fn intersection_size<'a>(
			&'a self,
			iter: impl Iterator<Item = &'a Sender> + 'a,
		) -> usize {
			let mut count = 0;
			for sender in iter {
				if self.members.contains(sender) {
					count += 1;
				}
			}
			count
		}
	}

	impl<Sender: PartialEq + Eq + Hash + Clone, Value: PartialEq + Eq + Hash + Clone + 'static>
		Subcommittee<Value> for TestSubcommittee<Sender>
	{
		fn condition<'a>(
			&'a self,
			partials: impl Iterator<Item = (&'a Self, &'a Value)> + 'a,
		) -> Condition<Value> {
			// Map all the senders to values.
			// If any sender maps to multiple values, this becomes Condition::Hung.
			let mut sender_to_values: HashMap<&Sender, &Value> = HashMap::new();
			for (subcommittee, value) in partials {
				for sender in subcommittee.senders() {
					match sender_to_values.get(sender) {
						Some(value) => {
							if value != value {
								return Condition::Hung;
							}
						}
						None => {
							sender_to_values.insert(sender, value);
						}
					}
				}
			}
			let senders_count = sender_to_values.len();

			// Invert the mapping to get values to senders.
			let mut values_to_senders: HashMap<&Value, Vec<&Sender>> = HashMap::new();
			for (sender, value) in sender_to_values {
				values_to_senders.entry(value).or_insert(Vec::new()).push(sender);
			}

			// Flatten see if there is a subcommittee satisfying 2f + 1 members.
			let mut max_senders = 0;
			let byzantine_quorum_size = self.byzantine_quorum_size();
			for (value, senders) in values_to_senders {
				max_senders = max_senders.max(senders.len());
				if senders.len() >= byzantine_quorum_size {
					return Condition::Consensus(value.clone());
				}
			}

			// If there aren't enough unaccounted for senders, return Condition::Hung.
			if (self.size() - senders_count) < (byzantine_quorum_size - max_senders) {
				return Condition::Hung;
			}

			// Otherwise, we're still in progress.
			Condition::InProgress
		}
	}

	#[test]
	fn test_subcommittee_consensus_condition() {
		let mut subcommittee = TestSubcommittee::<u32>::new();
		subcommittee.add_member(1);
		subcommittee.add_member(2);
		subcommittee.add_member(3);
		subcommittee.add_member(4);
		subcommittee.add_member(5);
		subcommittee.add_member(6);

		let condition = subcommittee.condition(vec![(&subcommittee, &1)].into_iter());

		assert_eq!(condition, Condition::Consensus(1));
	}
}
