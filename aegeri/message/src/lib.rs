pub mod certificate;
pub use certificate::*;

pub mod transaction;
pub use transaction::*;

pub mod block;
pub use block::*;

pub mod subcommittee;
pub use subcommittee::*;

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
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Id([u8; 32]);

impl Id {
	/// Builds a new ID from the payload bytes.
	fn new(payload_bytes: &[u8], nonce: &Nonce, public_key: &PublicKey) -> Self {
		let mut hasher = sha3::Sha3_256::new();
		hasher.update(payload_bytes);
		hasher.update(nonce.as_bytes());
		hasher.update(public_key.as_bytes());
		let hash: [u8; 32] = hasher.finalize().into();
		Self(hash)
	}

	/// Borrow the bytes of the ID.
	fn as_bytes(&self) -> &[u8] {
		&self.0
	}
}

/// The signer of a message.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct PublicKey(Vec<u8>);

impl PublicKey {
	/// Builds a signer from the given bytes.
	pub fn new(signer: &SigningKey<MlDsa44>) -> Self {
		let encoded_verifying_key = signer.verifying_key().encode();

		Self(encoded_verifying_key.as_slice().to_vec())
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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Message<P> {
	id: Id,
	public_key: PublicKey,
	signature: Signature,
	payload: P,
	nonce: Nonce,
}

impl<P> Message<P> {
	pub fn payload(&self) -> &P {
		&self.payload
	}

	pub fn nonce(&self) -> &Nonce {
		&self.nonce
	}

	pub fn id(&self) -> &Id {
		&self.id
	}

	pub fn public_key(&self) -> &PublicKey {
		&self.public_key
	}

	pub fn signature(&self) -> &Signature {
		&self.signature
	}
}

#[derive(Debug, Clone, thiserror::Error, PartialEq, Eq)]
pub enum VerificationError {
	#[error("Signature verification failed")]
	SignatureVerificationFailed,
	#[error("ID mismatch")]
	IdMismatch,
	#[error("Payload serialization failed")]
	PayloadSerializationFailed,
}

/// A Verified instance of a message.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct VerifiedMessage<P>(Message<P>);

impl<P> VerifiedMessage<P> {
	pub fn payload(&self) -> &P {
		&self.0.payload()
	}

	pub fn nonce(&self) -> &Nonce {
		&self.0.nonce()
	}

	pub fn id(&self) -> &Id {
		&self.0.id()
	}

	pub fn public_key(&self) -> &PublicKey {
		&self.0.public_key()
	}

	pub fn signature(&self) -> &Signature {
		&self.0.signature()
	}
}

impl<P: Serialize + for<'a> Deserialize<'a>> Message<P> {
	pub fn verify(&self) -> Result<(), VerificationError> {
		// Convert the public key to an encoded verifying key.
		let verifying_key = self.public_key().to_verifying_key()?;

		// Verify the signature.
		verifying_key
			.verify(self.id.as_bytes(), &self.signature().to_ml_dsa_signature()?)
			.map_err(|_| VerificationError::SignatureVerificationFailed)?;

		// Check that the id matches the hash
		let serialized_payload = serde_json::to_vec(&self.payload)
			.map_err(|_| VerificationError::PayloadSerializationFailed)?;
		let computed_id = Id::new(&serialized_payload, &self.nonce, &self.public_key());
		if computed_id != self.id {
			return Err(VerificationError::IdMismatch);
		}

		Ok(())
	}

	pub fn into_verified(self) -> Result<VerifiedMessage<P>, VerificationError> {
		self.verify()?;
		Ok(VerifiedMessage(self))
	}
}

#[derive(thiserror::Error, Debug, Clone)]
pub enum TransactionMessageError {
	#[error("Payload serialization failed")]
	PayloadSerializationFailed,
}

impl Message<Transaction> {
	/// Builds a new transaction message.
	pub fn try_new(
		signer: &SigningKey<MlDsa44>,
		payload: Transaction,
		nonce: Nonce,
	) -> Result<Self, TransactionMessageError> {
		let public_key = PublicKey::new(signer);
		let id = Id::new(
			&serde_json::to_vec(&payload)
				.map_err(|_| TransactionMessageError::PayloadSerializationFailed)?,
			&nonce,
			&public_key,
		);
		let signature = Signature::new(signer, &id);
		Ok(Self { id, public_key, signature, payload, nonce })
	}
}

impl Message<Certificate> {
	/// Builds a new certificate message.
	pub fn try_new(
		signer: &SigningKey<MlDsa44>,
		payload: Certificate,
		nonce: Nonce,
	) -> Result<Self, TransactionMessageError> {
		let public_key = PublicKey::new(signer);
		let id = Id::new(
			&serde_json::to_vec(&payload)
				.map_err(|_| TransactionMessageError::PayloadSerializationFailed)?,
			&nonce,
			&public_key,
		);
		let signature = Signature::new(signer, &id);
		Ok(Self { id, public_key, signature, payload, nonce })
	}
}

/// A unified message for designs wherein we want to
/// store the certificates and transactions together.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum UnifiedMessage {
	Transaction(Message<Transaction>),
	Certificate(Message<Certificate>),
}

impl From<Message<Transaction>> for UnifiedMessage {
	fn from(message: Message<Transaction>) -> Self {
		UnifiedMessage::Transaction(message)
	}
}

impl From<Message<Certificate>> for UnifiedMessage {
	fn from(message: Message<Certificate>) -> Self {
		UnifiedMessage::Certificate(message)
	}
}

#[cfg(test)]
mod tests {
	use ml_dsa::B32;

	use super::*;

	#[test]
	fn test_message_verifies() -> Result<(), anyhow::Error> {
		let signer = SigningKey::<MlDsa44>::from_seed(&B32::from_iter(vec![1; 32]));
		let transaction = Transaction::ElfScript(ElfScript::new(b"hello, world"));
		let nonce = Nonce::new(b"nonce");
		let message = Message::<Transaction>::try_new(&signer, transaction, nonce)?;
		message.verify()?;
		Ok(())
	}

	#[test]
	fn test_message_rejects_invalid_signature() -> Result<(), anyhow::Error> {
		let signer = SigningKey::<MlDsa44>::from_seed(&B32::from_iter(vec![1; 32]));
		let transaction = Transaction::ElfScript(ElfScript::new(b"hello, world"));
		let nonce = Nonce::new(b"nonce");
		let mut message = Message::<Transaction>::try_new(&signer, transaction, nonce)?;
		message.signature.0[0] = !message.signature.0[0];
		assert!(message.verify() == Err(VerificationError::SignatureVerificationFailed));
		Ok(())
	}
}
