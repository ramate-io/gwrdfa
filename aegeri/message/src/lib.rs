pub mod certificate;
pub use certificate::*;

pub mod transaction;
pub use transaction::*;

use ml_dsa::{
	signature::{SignatureEncoding, Signer},
	MlDsa44, SigningKey, VerifyingKey,
};
use serde::{Deserialize, Serialize};
use sha3::Digest;

/// The ID of a message
///
/// This will be a hash of the payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Id(Vec<u8>);

impl Id {
	/// Builds a new ID from the payload bytes.
	fn new(payload_bytes: &[u8], nonce: &Nonce) -> Self {
		let mut hasher = sha3::Sha3_256::new();
		hasher.update(payload_bytes);
		hasher.update(nonce.as_bytes());
		let hash = hasher.finalize();
		Self(hash.to_vec())
	}

	/// Borrow the bytes of the ID.
	fn as_bytes(&self) -> &[u8] {
		&self.0
	}
}

/// The signer of a message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicKey(Vec<u8>);

impl PublicKey {
	/// Builds a signer from the given bytes.
	pub fn new(public_key: impl Into<Vec<u8>>) -> Self {
		Self(public_key.into())
	}

	/// Borrow the bytes of the signer.
	fn as_bytes(&self) -> &[u8] {
		&self.0
	}
}

/// The signature of a message for a given signer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signature(Vec<u8>);

impl Signature {
	/// Builds a signature from the given bytes.
	pub fn new(signer: &SigningKey<MlDsa44>, id: &Id) -> Self {
		let signature = signer.sign(id.as_bytes());
		Self(signature.to_bytes().to_vec())
	}

	/// Borrow the bytes of the signature.
	fn as_bytes(&self) -> &[u8] {
		&self.0
	}
}

/// The nonce of a message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Nonce(Vec<u8>);

impl Nonce {
	/// Builds a nonce from the given bytes.
	pub fn new(nonce: impl Into<Vec<u8>>) -> Self {
		Self(nonce.into())
	}

	/// Borrow the bytes of the nonce.
	fn as_bytes(&self) -> &[u8] {
		&self.0
	}
}

/// The message itself.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message<P> {
	id: Id,
	signer: PublicKey,
	signature: Signature,
	payload: P,
	nonce: Nonce,
}

#[derive(Debug, thiserror::Error)]
pub enum VerificationError {
	#[error("Signature verification failed")]
	SignatureVerificationFailed,
}

/// A Verified instance of a message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifiedMessage<P>(Message<P>);

impl<P> Message<P> {
	pub fn verify(&self, signer: &PublicKey) -> Result<(), VerificationError> {
		let verifying_key = VerifyingKey::<MlDsa44>::decode(signer.as_bytes().into_array())
			.map_err(|_| VerificationError::SignatureVerificationFailed)?;

		Ok(())
	}
}
