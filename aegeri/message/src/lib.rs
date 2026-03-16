pub mod certificate;
pub use certificate::*;

pub mod transaction;
pub use transaction::*;

pub mod block;
pub use block::*;

pub mod subcommittee;
pub use subcommittee::*;

pub use gossamer::{GossamerMessage, GossamerMessageError};

#[cfg(test)]
use ml_dsa::B32;
use ml_dsa::{
	signature::{SignatureEncoding, Signer, Verifier},
	EncodedSignature, EncodedVerifyingKey, MlDsa44, Signature as MlDsaSignature, SigningKey,
	VerifyingKey,
};
use serde::{Deserialize, Serialize};
use sha3::Digest;
use std::fmt;
use std::fmt::Write as _;

fn write_lower_hex(f: &mut fmt::Formatter<'_>, bytes: &[u8]) -> fmt::Result {
	const HEX: &[u8; 16] = b"0123456789abcdef";
	for &byte in &bytes[..16] {
		let hi = HEX[(byte >> 4) as usize] as char;
		let lo = HEX[(byte & 0x0f) as usize] as char;
		f.write_char(hi)?;
		f.write_char(lo)?;
	}
	Ok(())
}

/// The ID of a message
///
/// This will be a hash of the payload.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

impl fmt::Display for Id {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write_lower_hex(f, self.as_bytes())
	}
}

/// The signer of a message.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PublicKey(Vec<u8>);

impl PublicKey {
	#[cfg(test)]
	pub fn new_for_test(seed: u8) -> Self {
		let signer = SigningKey::<MlDsa44>::from_seed(&B32::from_iter(vec![seed; 32]));
		Self::new(&signer)
	}

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

impl fmt::Display for PublicKey {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write_lower_hex(f, self.as_bytes())
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

impl fmt::Display for Signature {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write_lower_hex(f, self.as_bytes())
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

impl VerifiedMessage<Certificate> {
	/// When inserting into the buffer, we will often need to extract the index, subcommittee, and proposal.
	/// We don't typically have to do anything similar for Transactions since the
	/// entire enveloped is what we're coming to consensus on.
	pub fn into_consensus_parts(self) -> (Index, AegeriSubcommittee, Proposal) {
		let Message { id: _, public_key, signature: _, payload, nonce: _ } = self.0;
		let Certificate { index, value: proposal } = payload;
		let subcommittee = AegeriSubcommittee::new(index).with_members(std::iter::once(public_key));
		(index, subcommittee, proposal)
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

impl GossamerMessage for UnifiedMessage {
	fn to_gossamer_bytes(&self) -> Result<Vec<u8>, GossamerMessageError> {
		let bytes = serde_json::to_vec(self)
			.map_err(|e| GossamerMessageError::SerializeError((e.to_string(), vec![])))?;
		Ok(bytes)
	}

	fn from_gossamer_bytes(bytes: Vec<u8>) -> Result<Self, GossamerMessageError> {
		let message = serde_json::from_slice(&bytes)
			.map_err(|e| GossamerMessageError::DeserializeError((e.to_string(), bytes)))?;
		Ok(message)
	}
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

	#[test]
	fn test_hex_display_for_id_public_key_and_signature() -> Result<(), anyhow::Error> {
		let signer = SigningKey::<MlDsa44>::from_seed(&B32::from_iter(vec![7; 32]));
		let transaction = Transaction::ElfScript(ElfScript::new(b"display"));
		let nonce = Nonce::new(b"nonce");
		let message = Message::<Transaction>::try_new(&signer, transaction, nonce)?;

		let id_hex = message.id().to_string();
		let pk_hex = message.public_key().to_string();
		let sig_hex = message.signature().to_string();

		assert_eq!(id_hex.len(), 32);
		assert_eq!(pk_hex.len() % 2, 0);
		assert_eq!(sig_hex.len() % 2, 0);
		assert!(id_hex.chars().all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase()));
		assert!(pk_hex.chars().all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase()));
		assert!(sig_hex.chars().all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase()));
		Ok(())
	}
}
