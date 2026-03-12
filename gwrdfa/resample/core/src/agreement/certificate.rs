//! Certificate abstractions for resample agreement evaluation.

use super::Subcommittee;
use parabyzantine::NoOp;

/// Certificate index keyed view used during agreement evaluation.
///
/// Implementations store `(index, value, subcommittee)` certificate evidence and
/// can stream partial evidence for a specific index.
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

#[cfg(any(test, feature = "std"))]
pub mod test {
	use super::CertificateSet;
	use crate::agreement::std::MemoryCertificateSet;

	#[test]
	fn test_certificate_set() {
		use crate::agreement::std::VoterSet;

		let mut cert_set = MemoryCertificateSet::new();

		let mut subcommittee_1 = VoterSet::new();
		subcommittee_1.add_member(1);
		subcommittee_1.add_member(2);
		subcommittee_1.add_member(3);
		subcommittee_1.add_member(4);
		subcommittee_1.add_member(5);
		subcommittee_1.add_member(6);

		cert_set.insert(1, 2, subcommittee_1.clone());

		let mut subcommittee_2 = VoterSet::new();
		subcommittee_2.add_member(7);
		subcommittee_2.add_member(8);
		subcommittee_2.add_member(9);
		subcommittee_2.add_member(10);
		subcommittee_2.add_member(11);
		subcommittee_2.add_member(12);

		cert_set.insert(2, 3, subcommittee_2.clone());

		let mut subcommittee_3 = VoterSet::new();
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

	pub type TestCertificateSet<Index, Value, Sub> = MemoryCertificateSet<Index, Value, Sub>;
}
