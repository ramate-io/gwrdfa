#![no_std]

pub mod agreement;
pub mod buffer;
pub mod certificate;
pub mod chunk;
pub mod index;
pub mod transaction;

use buffer::Bufferlike;
use core::marker::PhantomData;

pub trait ParabyzantineSpec {
	/// A buffer of messages from outside the system.
	/// Often, these are user messages.
	type TransactionBuffer: Bufferlike;

	/// A buffer of messages from within the system.
	/// These are messages from other processes.
	type CertificateBuffer: Bufferlike;

	/// A buffer representing agreements in the protocol.
	type AgreementBuffer: Bufferlike;

	/// A buffer representing tasks in the protocol.
	type TaskBuffer: Bufferlike;

	/// A buffer representing broadcasts in the protocol.
	/// These are messages we are going to send to the oustide world.
	/// Often, these will have certificates attached to them.
	type BroadcastBuffer: Bufferlike;

	/// A general purpose buffer that is always available.
	type MetadataBuffer: Bufferlike;
}

pub trait Parabyzantine<Spec: ParabyzantineSpec>: Sized {
	/// Prepares all of the buffer.
	fn prepare_buffers(
		&mut self,
		transaction_buffer: &mut Spec::TransactionBuffer,
		certificate_buffer: &mut Spec::CertificateBuffer,
		agreement_buffer: &mut Spec::AgreementBuffer,
		broadcast_buffer: &mut Spec::BroadcastBuffer,
		metadata_buffer: &mut Spec::MetadataBuffer,
	);

	/// Processes certificates into agreements.
	fn process_certificates_into_agreements(
		&mut self,
		certificate_buffer: &Spec::CertificateBuffer,
		agreement_buffer: &mut Spec::AgreementBuffer,
	);

	/// Processes agreements and transactions into tasks.
	fn process_agreements_and_transactions_into_tasks(
		&mut self,
		agreement_buffer: &Spec::AgreementBuffer,
		transaction_buffer: &Spec::TransactionBuffer,
		task_buffer: &mut Spec::TaskBuffer,
	);

	/// Processes tasks into broadcasts.
	fn process_tasks_into_broadcasts(
		&mut self,
		task_buffer: &Spec::TaskBuffer,
		broadcast_buffer: &mut Spec::BroadcastBuffer,
	);

	/// Processes broadcasts, often this invovles dispatching to some other system.
	fn process_broadcasts(&mut self, broadcast_buffer: &Spec::BroadcastBuffer);

	/// Finalizes the buffers.
	fn finalize_buffers(
		&mut self,
		transaction_buffer: &mut Spec::TransactionBuffer,
		certificate_buffer: &mut Spec::CertificateBuffer,
		agreement_buffer: &mut Spec::AgreementBuffer,
		broadcast_buffer: &mut Spec::BroadcastBuffer,
		metadata_buffer: &mut Spec::MetadataBuffer,
	);

	fn finalize(&mut self);

	fn compose<Other: Parabyzantine<Spec>>(self, other: Other) -> Composition<Spec, Self, Other> {
		Composition::new(self, other)
	}
}

pub struct Composition<
	Spec: ParabyzantineSpec,
	Left: Parabyzantine<Spec>,
	Right: Parabyzantine<Spec>,
> {
	__phantom: PhantomData<Spec>,
	left: Left,
	right: Right,
}

impl<Spec: ParabyzantineSpec, Left: Parabyzantine<Spec>, Right: Parabyzantine<Spec>>
	Composition<Spec, Left, Right>
{
	pub fn new(left: Left, right: Right) -> Self {
		Self { left, right, __phantom: PhantomData }
	}
}

impl<Spec: ParabyzantineSpec, Left: Parabyzantine<Spec>, Right: Parabyzantine<Spec>>
	Parabyzantine<Spec> for Composition<Spec, Left, Right>
{
	fn prepare_buffers(
		&mut self,
		transaction_buffer: &mut Spec::TransactionBuffer,
		certificate_buffer: &mut Spec::CertificateBuffer,
		agreement_buffer: &mut Spec::AgreementBuffer,
		broadcast_buffer: &mut Spec::BroadcastBuffer,
		metadata_buffer: &mut Spec::MetadataBuffer,
	) {
		self.left.prepare_buffers(
			transaction_buffer,
			certificate_buffer,
			agreement_buffer,
			broadcast_buffer,
			metadata_buffer,
		);
		self.right.prepare_buffers(
			transaction_buffer,
			certificate_buffer,
			agreement_buffer,
			broadcast_buffer,
			metadata_buffer,
		);
	}

	fn process_transactions(
		&mut self,
		transaction_buffer: &Spec::TransactionBuffer,
		certificate_buffer: &mut Spec::CertificateBuffer,
		agreement_buffer: &mut Spec::AgreementBuffer,
		broadcast_buffer: &mut Spec::BroadcastBuffer,
		metadata_buffer: &mut Spec::MetadataBuffer,
	) {
		self.left.process_transactions(
			transaction_buffer,
			certificate_buffer,
			agreement_buffer,
			broadcast_buffer,
			metadata_buffer,
		);
		self.right.process_transactions(
			transaction_buffer,
			certificate_buffer,
			agreement_buffer,
			broadcast_buffer,
			metadata_buffer,
		);
	}

	fn process_certificates(
		&mut self,
		certificate_buffer: &Spec::CertificateBuffer,
		agreement_buffer: &mut Spec::AgreementBuffer,
		broadcast_buffer: &mut Spec::BroadcastBuffer,
		metadata_buffer: &mut Spec::MetadataBuffer,
	) {
		self.left.process_certificates(
			certificate_buffer,
			agreement_buffer,
			broadcast_buffer,
			metadata_buffer,
		);
		self.right.process_certificates(
			certificate_buffer,
			agreement_buffer,
			broadcast_buffer,
			metadata_buffer,
		);
	}

	fn process_agreements(
		&mut self,
		agreement_buffer: &Spec::AgreementBuffer,
		broadcast_buffer: &mut Spec::BroadcastBuffer,
		metadata_buffer: &mut Spec::MetadataBuffer,
	) {
		self.left
			.process_agreements(agreement_buffer, broadcast_buffer, metadata_buffer);
		self.right
			.process_agreements(agreement_buffer, broadcast_buffer, metadata_buffer);
	}

	fn process_broadcasts(
		&mut self,
		broadcast_buffer: &Spec::BroadcastBuffer,
		metadata_buffer: &mut Spec::MetadataBuffer,
	) {
		self.left.process_broadcasts(broadcast_buffer, metadata_buffer);
		self.right.process_broadcasts(broadcast_buffer, metadata_buffer);
	}

	fn finalize_buffers(
		&mut self,
		transaction_buffer: &mut Spec::TransactionBuffer,
		certificate_buffer: &mut Spec::CertificateBuffer,
		agreement_buffer: &mut Spec::AgreementBuffer,
		broadcast_buffer: &mut Spec::BroadcastBuffer,
		metadata_buffer: &mut Spec::MetadataBuffer,
	) {
		self.left.finalize_buffers(
			transaction_buffer,
			certificate_buffer,
			agreement_buffer,
			broadcast_buffer,
			metadata_buffer,
		);
		self.right.finalize_buffers(
			transaction_buffer,
			certificate_buffer,
			agreement_buffer,
			broadcast_buffer,
			metadata_buffer,
		);
	}

	fn finalize(&mut self) {
		self.left.finalize();
		self.right.finalize();
	}
}
