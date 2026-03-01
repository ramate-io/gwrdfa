use serde::{Deserialize, Serialize};

/// The ELF binary of a transaction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Elf(Vec<u8>);

/// The payload of a transaction.
///
/// Users can either send an ELF binaary to execute, or a join intent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Transaction {
	/// An ELF encoded program to run.
	Elf(Elf),
	/// The intent to join the quorum.
	Join,
}
