use super::Subcommittee;
use parabyzantine::{NoOp, NO_OP};

pub trait Certificate<Index: Eq, Value: Eq + 'static, Sub: Subcommittee<Value>>: Eq {
	/// The index of the message.
	fn index(&self) -> Index;

	/// The value of the message.
	fn value(&self) -> Value;

	/// The subcommittee of the message.
	fn subcommittee(&self) -> &Sub;
}

pub trait CertificateSet<
	Index: Eq,
	Value: Eq + 'static,
	Item: Certificate<Index, Value, Sub>,
	Sub: Subcommittee<Value>,
>
{
	fn insert(&mut self, item: Item);

	fn remove(&mut self, item: Item);

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

/// A [Certificate] for the [NoOp] struct.
impl Certificate<NoOp, NoOp, NoOp> for NoOp {
	fn index(&self) -> NoOp {
		NoOp
	}
	fn value(&self) -> NoOp {
		NoOp
	}
	fn subcommittee(&self) -> &NoOp {
		&NO_OP
	}
}

/// A [CertificateSet] for the [NoOp] struct.
impl CertificateSet<NoOp, NoOp, NoOp, NoOp> for NoOp {
	fn insert(&mut self, _item: NoOp) {}
	fn remove(&mut self, _item: NoOp) {}
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Cert<Index: Eq, Value: Eq + 'static, Sub: Subcommittee<Value>> {
	pub index: Index,
	pub value: Value,
	pub subcommittee: Sub,
}

impl<Index: Eq + Copy, Value: Eq + Copy, Sub: Subcommittee<Value>> Certificate<Index, Value, Sub>
	for Cert<Index, Value, Sub>
{
	fn index(&self) -> Index {
		self.index
	}
	fn value(&self) -> Value {
		self.value
	}

	fn subcommittee(&self) -> &Sub {
		&self.subcommittee
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
		> CertificateSet<Index, Value, Cert<Index, Value, Sub>, Sub>
		for TestCertificateSet<Index, Value, Sub>
	{
		fn insert(&mut self, item: Cert<Index, Value, Sub>) {
			self.certs
				.entry(item.index)
				.or_insert(HashSet::new())
				.insert((item.subcommittee, item.value));
		}
		fn remove(&mut self, item: Cert<Index, Value, Sub>) {
			self.certs
				.entry(item.index)
				.or_insert(HashSet::new())
				.remove(&(item.subcommittee, item.value));
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
	fn test_cert_index_and_value() {
		let cert = Cert { index: 1, value: 2, subcommittee: NoOp };
		assert_eq!(cert.index(), 1);
		assert_eq!(cert.value(), 2);
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

		cert_set.insert(Cert { index: 1, value: 2, subcommittee: subcommittee_1.clone() });

		let mut subcommittee_2 = TestSubcommittee::new();
		subcommittee_2.add_member(7);
		subcommittee_2.add_member(8);
		subcommittee_2.add_member(9);
		subcommittee_2.add_member(10);
		subcommittee_2.add_member(11);
		subcommittee_2.add_member(12);

		cert_set.insert(Cert { index: 2, value: 3, subcommittee: subcommittee_2.clone() });

		let mut subcommittee_3 = TestSubcommittee::new();
		subcommittee_3.add_member(13);
		subcommittee_3.add_member(14);
		subcommittee_3.add_member(15);
		subcommittee_3.add_member(16);
		subcommittee_3.add_member(17);
		subcommittee_3.add_member(18);

		cert_set.insert(Cert { index: 3, value: 4, subcommittee: subcommittee_3.clone() });

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
