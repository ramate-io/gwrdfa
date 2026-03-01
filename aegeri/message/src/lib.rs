pub mod certificate;
pub use certificate::*;

pub mod transaction;
pub use transaction::*;

/// The ID of a message
///
/// This will be a hash of the payload.
#[derive(Debug, Clone)]
pub struct Id(Vec<u8>);

/// The signer of a message.
#[derive(Debug, Clone)]
pub struct Signer(Vec<u8>);

/// The signature of a message for a given signer.
#[derive(Debug, Clone)]
pub struct Signature(Vec<u8>);

/// The unified message payload for broadcasts on the system.
#[derive(Debug, Clone)]
pub enum Payload {
	/// A transaction from outside the system.
	Transaction(Transaction),
	/// A certificate from inside the system.
	Certificate(Certificate),
}

/// The message itself.
#[derive(Debug, Clone)]
pub struct Message {
	id: Id,
	signer: Signer,
	signature: Signature,
	payload: Payload,
}
