pub mod container;

/// A wrapper for an index s.t. we guarantee disjoint types.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Index<T: Eq>(pub T);

/// A wrapper for a value s.t. we guarantee disjoint types.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Value<T: Eq + 'static>(pub T);

/// A wrapper for a subcommittee s.t. we guarantee disjoint types.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Sub<T: Eq>(pub T);
