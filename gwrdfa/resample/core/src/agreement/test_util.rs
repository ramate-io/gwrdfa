pub mod container;
use crate::agreement::{Condition, Subcommittee};

/// A wrapper for an index s.t. we guarantee disjoint types.
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Index<T: Eq>(pub T);

/// A wrapper for a value s.t. we guarantee disjoint types.
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Value<T: Eq + 'static>(pub T);

/// A wrapper for a subcommittee s.t. we guarantee disjoint types.
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Sub<T: Eq>(pub T);

impl<T: Eq + 'static + Subcommittee<V>, V: Eq + 'static> Subcommittee<Value<V>> for Sub<T> {
	fn condition<'a>(
		&'a self,
		partials: impl Iterator<Item = (&'a Self, &'a Value<V>)> + 'a,
	) -> Condition<Value<V>> {
		self.0
			.condition(partials.map(|(subcommittee, value)| (&subcommittee.0, &value.0)))
			.map(|value| Value(value))
	}
}

pub trait TextIndexabled: Sized {
	fn next(&self) -> Option<Self>;
}

impl TextIndexabled for u32 {
	fn next(&self) -> Option<Self> {
		Some(self + 1)
	}
}

impl<T: Eq + 'static + TextIndexabled> TextIndexabled for Index<T> {
	fn next(&self) -> Option<Self> {
		self.0.next().map(|index| Index(index))
	}
}
