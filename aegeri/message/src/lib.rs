pub mod certificate;
pub use certificate::*;

pub mod transaction;
pub use transaction::*;

use ml_dsa::{
	signature::{SignatureEncoding, Signer, Verifier},
	EncodedSignature, EncodedVerifyingKey, MlDsa44, Signature as MlDsaSignature, SigningKey,
	VerifyingKey,
};
use serde::{Deserialize, Serialize};
use sha3::Digest;

/// The ID of a message
///
/// This will be a hash of the payload.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
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

	/// Convert the public key to an encoded verifying key.
	pub fn to_encoded_verifying_key(
		&self,
	) -> Result<EncodedVerifyingKey<MlDsa44>, VerificationError> {
		EncodedVerifyingKey::<MlDsa44>::try_from_iter(self.as_bytes().iter().copied().into_iter())
			.map_err(|_| VerificationError::SignatureVerificationFailed)
	}

	/// Convert the public key to a verifying key.
	pub fn to_verifying_key(&self) -> Result<VerifyingKey<MlDsa44>, VerificationError> {
		let encoded_verifying_key = self.to_encoded_verifying_key()?;
		Ok(VerifyingKey::<MlDsa44>::decode(&encoded_verifying_key))
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

	/// Convert the signature to an encoded signature.
	pub fn to_encoded_signature(&self) -> Result<EncodedSignature<MlDsa44>, VerificationError> {
		EncodedSignature::<MlDsa44>::try_from_iter(self.as_bytes().iter().copied().into_iter())
			.map_err(|_| VerificationError::SignatureVerificationFailed)
	}

	/// Convert the signature to an ML-DSA signature.
	pub fn to_ml_dsa_signature(&self) -> Result<MlDsaSignature<MlDsa44>, VerificationError> {
		MlDsaSignature::<MlDsa44>::decode(&self.to_encoded_signature()?)
			.ok_or(VerificationError::SignatureVerificationFailed)
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
	#[error("ID mismatch")]
	IdMismatch,
	#[error("Payload serialization failed")]
	PayloadSerializationFailed,
}

/// A Verified instance of a message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifiedMessage<P>(Message<P>);

impl<P: Serialize + for<'a> Deserialize<'a>> Message<P> {
	pub fn verify(&self, public_key: &PublicKey) -> Result<(), VerificationError> {
		// Convert the public key to an encoded verifying key.
		let verifying_key = public_key.to_verifying_key()?;

		// Verify the signature.
		verifying_key
			.verify(self.id.as_bytes(), &self.signature.to_ml_dsa_signature()?)
			.map_err(|_| VerificationError::SignatureVerificationFailed)?;

		// Check that the id matches the hash
		let serialized_payload = serde_json::to_vec(&self.payload)
			.map_err(|_| VerificationError::PayloadSerializationFailed)?;
		let computed_id = Id::new(&serialized_payload, &self.nonce);
		if computed_id != self.id {
			return Err(VerificationError::IdMismatch);
		}

		Ok(())
	}
}
