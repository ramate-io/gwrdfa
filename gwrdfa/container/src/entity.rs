#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct ContainerEntity(usize);

impl ContainerEntity {
	pub fn new(index: usize) -> Self {
		Self(index)
	}

	/// Gets the index of the entity.
	pub fn index(&self) -> usize {
		self.0
	}

	/// Gets the next entity in the sequence.
	///
	/// NOTE: we may update this to check max in the future,
	/// but that is an extreme edge case.
	pub fn next(&self) -> Self {
		Self(self.0 + 1)
	}
}
