use super::Subcommittee;
use parabyzantine::NoOp;

pub trait CertificateSet<Index: Eq, Value: Eq + 'static, Sub: Subcommittee<Value>> {
	fn insert(&mut self, index: Index, value: Value, subcommittee: Sub);

	fn remove(&mut self, index: Index, value: Value, subcommittee: Sub);

	/// Returns an iterator of tuples mapping subcommittees to values for a given index.
	///
	/// NOTE: this does not assume compaction.
	fn partial_subcommittees_for_index<'a>(
		&'a self,
		index: &'a Index,
	) -> impl Iterator<Item = (&'a Sub, &'a Value)> + 'a
	where
		Self: 'a,
		Sub: 'a,
		Value: 'a;
}

/// A [CertificateSet] for the [NoOp] struct.
impl CertificateSet<NoOp, NoOp, NoOp> for NoOp {
	fn insert(&mut self, _index: NoOp, _value: NoOp, _subcommittee: NoOp) {}
	fn remove(&mut self, _index: NoOp, _value: NoOp, _subcommittee: NoOp) {}
	fn partial_subcommittees_for_index<'a>(
		&'a self,
		_index: &'a NoOp,
	) -> impl Iterator<Item = (&'a NoOp, &'a NoOp)> + 'a
	where
		Self: 'a,
		NoOp: 'a,
	{
		[].into_iter()
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::agreement::subcommittee::test::TestSubcommittee;
	use std::collections::{HashMap, HashSet};
	use std::hash::Hash;
	use std::vec;
	use std::vec::Vec;

	#[derive(Debug)]
	pub struct TestCertificateSet<
		Index: Eq + Copy + Hash,
		Value: Eq + Copy + 'static + Hash,
		Sub: Subcommittee<Value> + Hash,
	> {
		certs: HashMap<Index, HashSet<(Sub, Value)>>,
	}

	impl<
			Index: Eq + Copy + Hash,
			Value: Eq + Copy + 'static + Hash,
			Sub: Subcommittee<Value> + Hash,
		> TestCertificateSet<Index, Value, Sub>
	{
		pub fn new() -> Self {
			Self { certs: HashMap::new() }
		}
	}

	impl<
			Index: Eq + Copy + Hash,
			Value: Eq + Copy + 'static + Hash,
			Sub: Subcommittee<Value> + Hash,
		> CertificateSet<Index, Value, Sub> for TestCertificateSet<Index, Value, Sub>
	{
		fn insert(&mut self, index: Index, value: Value, subcommittee: Sub) {
			self.certs.entry(index).or_insert(HashSet::new()).insert((subcommittee, value));
		}
		fn remove(&mut self, index: Index, value: Value, subcommittee: Sub) {
			self.certs.entry(index).or_insert(HashSet::new()).remove(&(subcommittee, value));
		}

		fn partial_subcommittees_for_index<'a>(
			&'a self,
			index: &'a Index,
		) -> impl Iterator<Item = (&'a Sub, &'a Value)> + 'a
		where
			Self: 'a,
			Sub: 'a,
			Value: 'a,
		{
			self.certs
				.iter()
				.filter_map(move |(i, certs)| {
					if i == index {
						Some(certs.iter().map(|(sub, v)| (sub, v)).collect::<Vec<_>>().into_iter())
					} else {
						None
					}
				})
				.flatten()
		}
	}

	#[test]
	fn test_certificate_set() {
		let mut cert_set = TestCertificateSet::new();

		let mut subcommittee_1 = TestSubcommittee::new();
		subcommittee_1.add_member(1);
		subcommittee_1.add_member(2);
		subcommittee_1.add_member(3);
		subcommittee_1.add_member(4);
		subcommittee_1.add_member(5);
		subcommittee_1.add_member(6);

		cert_set.insert(1, 2, subcommittee_1.clone());

		let mut subcommittee_2 = TestSubcommittee::new();
		subcommittee_2.add_member(7);
		subcommittee_2.add_member(8);
		subcommittee_2.add_member(9);
		subcommittee_2.add_member(10);
		subcommittee_2.add_member(11);
		subcommittee_2.add_member(12);

		cert_set.insert(2, 3, subcommittee_2.clone());

		let mut subcommittee_3 = TestSubcommittee::new();
		subcommittee_3.add_member(13);
		subcommittee_3.add_member(14);
		subcommittee_3.add_member(15);
		subcommittee_3.add_member(16);
		subcommittee_3.add_member(17);
		subcommittee_3.add_member(18);

		cert_set.insert(3, 4, subcommittee_3.clone());

		// Check the first index
		let partial_subcommittees =
			cert_set.partial_subcommittees_for_index(&1).collect::<Vec<_>>();
		assert_eq!(partial_subcommittees, vec![(&subcommittee_1, &2)]);

		// Check there are no more subcommittees.
		let partial_subcommittees =
			cert_set.partial_subcommittees_for_index(&2).collect::<Vec<_>>();
		assert_eq!(partial_subcommittees, vec![(&subcommittee_2, &3)]);

		// Check the third index
		let partial_subcommittees =
			cert_set.partial_subcommittees_for_index(&3).collect::<Vec<_>>();
		assert_eq!(partial_subcommittees, vec![(&subcommittee_3, &4)]);
	}
}
