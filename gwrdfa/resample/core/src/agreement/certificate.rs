use super::Subcommittee;
use parabyzantine::NoOp;

pub trait Certificate<Index: Eq, Value: Eq + 'static, Sub: Subcommittee<Value>>:
	Eq + Sized
{
	/// The index of the message.
	fn index(&self) -> Index;

	/// The value of the message.
	fn value(&self) -> Value;

	/// The subcommittee of the message.
	fn subcommittee(&self) -> Sub;
}

pub trait CertificateSet<
	Index: Eq,
	Value: Eq + 'static,
	Item: Certificate<Index, Value, Sub>,
	Sub: Subcommittee<Value>,
>: Eq + Sized
{
	fn insert(&mut self, item: Item);

	fn remove(&mut self, item: Item);

	/// Returns an iterator of tuples mapping subcommittees to values for a given index.
	///
	/// NOTE: this does not assume compaction.
	fn partial_subcommittees_for_index<'a>(
		&'a self,
		index: &Index,
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
	fn subcommittee(&self) -> NoOp {
		NoOp
	}
}

/// A [CertificateSet] for the [NoOp] struct.
impl CertificateSet<NoOp, NoOp, NoOp, NoOp> for NoOp {
	fn insert(&mut self, _item: NoOp) {}
	fn remove(&mut self, _item: NoOp) {}
	fn partial_subcommittees_for_index<'a>(
		&'a self,
		_index: &NoOp,
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

impl<Index: Eq + Copy, Value: Eq + Copy, Sub: Subcommittee<Value> + Copy>
	Certificate<Index, Value, Sub> for Cert<Index, Value, Sub>
{
	fn index(&self) -> Index {
		self.index
	}
	fn value(&self) -> Value {
		self.value
	}

	fn subcommittee(&self) -> Sub {
		self.subcommittee
	}
}

#[cfg(test)]
mod test {
	use super::super::subcommittee::test::TestSubcommittee;
	use super::*;
	use std::collections::HashMap;
	use std::hash::Hash;

	pub struct TestCertificateSet<
		Index: Eq + Copy + Hash,
		Value: Eq + Copy + 'static + Hash,
		Sub: Subcommittee<Value> + Copy + Hash,
	> {
		pub certs: HashMap<Index, (Sub, Value)>,
	}

	impl<
			Index: Eq + Copy + Hash,
			Value: Eq + Copy + 'static + Hash,
			Sub: Subcommittee<Value> + Copy + Hash,
		> CertificateSet<Index, Value, Cert<Index, Value, Sub>, Sub>
		for TestCertificateSet<Index, Value, Sub>
	{
		fn insert(&mut self, item: Cert<Index, Value, Sub>) {
			self.certs.insert(item.index, (item.subcommittee, item.value));
		}
		fn remove(&mut self, item: Cert<Index, Value, Sub>) {
			self.certs.remove(&item.index);
		}

		fn partial_subcommittees_for_index<'a>(
			&'a self,
			index: &Index,
		) -> impl Iterator<Item = (&'a Sub, &'a Value)> + 'a
		where
			Self: 'a,
			Sub: 'a,
			Value: 'a,
		{
			self.certs.iter().filter_map(
				|(i, (sub, v))| {
					if i == index {
						Some((sub, v))
					} else {
						None
					}
				},
			)
		}
	}

	#[test]
	fn test_cert_index_and_value() {
		let cert = Cert { index: 1, value: 2 };
		assert_eq!(cert.index(), 1);
		assert_eq!(cert.value(), 2);
	}
}
