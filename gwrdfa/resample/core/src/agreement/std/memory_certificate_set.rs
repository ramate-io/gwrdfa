use crate::agreement::{CertificateSet, Subcommittee};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

#[derive(Debug)]
pub struct MemoryCertificateSet<
	Index: Eq + Hash,
	Value: Eq + 'static + Hash,
	Sub: Subcommittee<Value> + Hash,
> {
	certs: HashMap<Index, HashSet<(Sub, Value)>>,
}

impl<Index: Eq + Hash, Value: Eq + 'static + Hash, Sub: Subcommittee<Value> + Hash>
	MemoryCertificateSet<Index, Value, Sub>
{
	pub fn new() -> Self {
		Self { certs: HashMap::new() }
	}
}

impl<Index: Eq + Hash, Value: Eq + 'static + Hash, Sub: Subcommittee<Value> + Hash>
	CertificateSet<Index, Value, Sub> for MemoryCertificateSet<Index, Value, Sub>
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
