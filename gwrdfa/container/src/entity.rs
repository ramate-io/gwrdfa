#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct ContainerEntity(usize);

impl ContainerEntity {
	pub fn new(index: usize) -> Self {
		Self(index)
	}

	pub fn index(&self) -> usize {
		self.0
	}

	pub fn next(&self) -> Self {
		Self(self.0 + 1)
	}
}
