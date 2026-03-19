#![no_std]
#![no_main]
use fuste::{signer_at_index, Bytes, SignerStoreSystem};

#[fuste::main]
fn main() -> Result<(), ()> {
	let signer_store = SignerStoreSystem::canonical();
	let signer = signer_at_index(0).map_err(|_| ())?;

	// signer stores are index by multiple signers
	// for a single signer use an array of length 1
	signer_store.store([signer.clone()], Bytes(*b"Hello, world!")).map_err(|_| ())?;

	// to represent state that requires multiple signers, use a tuple
	let second_signer = signer_at_index(1).map_err(|_| ())?;
	signer_store
		.store([signer, second_signer], Bytes(*b"Hello, world!\n\t- From two signers"))
		.map_err(|_| ())?;

	Ok(())
}
