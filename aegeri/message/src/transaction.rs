use serde::{Deserialize, Serialize};

/// The ELF binary of a transaction.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct ElfScript(Vec<u8>);

impl ElfScript {
	pub fn new(elf: impl Into<Vec<u8>>) -> Self {
		Self(elf.into())
	}

	pub fn as_bytes(&self) -> &[u8] {
		&self.0
	}
}

/// The payload of a transaction.
///
/// Users can either send an ELF binaary to execute, or a join intent.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Transaction {
	/// An ELF encoded program to run.
	ElfScript(ElfScript),
	/// The intent to join the quorum.
	Join,
	/// The intent to leave the quorum.
	Leave,
}
