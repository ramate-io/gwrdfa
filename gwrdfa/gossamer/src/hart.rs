use crate::{Gossamer, GossamerMessage, GossamerMessageError};
use core::marker::PhantomData;
use parabyzantine::{
	buffer::Bundle,
	hart::{
		ParabyzantineDataBinding, ParabyzantineDataSpec, ParabyzantineHart, ParabyzantineWorld,
	},
};

pub struct GossamerHart<Binding: ParabyzantineDataBinding, Message: GossamerMessage> {
	__marker: PhantomData<(Binding, Message)>,
	gossamer: Gossamer,
	max_batch_size: usize,
}

impl<Binding: ParabyzantineDataBinding, Message: GossamerMessage> From<Gossamer>
	for GossamerHart<Binding, Message>
{
	fn from(gossamer: Gossamer) -> Self {
		Self::new(gossamer)
	}
}

impl<Binding: ParabyzantineDataBinding, Message: GossamerMessage> GossamerHart<Binding, Message> {
	pub fn new(gossamer: Gossamer) -> Self {
		Self { __marker: PhantomData, gossamer, max_batch_size: 256 }
	}

	pub fn with_max_batch_size(mut self, max_batch_size: usize) -> Self {
		self.max_batch_size = max_batch_size;
		self
	}
}

impl<Binding: ParabyzantineDataBinding, Message: GossamerMessage> ParabyzantineHart
	for GossamerHart<Binding, Message>
where
	Message: Bundle<
		<Binding::Spec as ParabyzantineDataSpec>::MessageEntity,
		<Binding::Spec as ParabyzantineDataSpec>::MessageBuffer,
	>,
	GossamerMessageError: Bundle<
		<Binding::Spec as ParabyzantineDataSpec>::MessageEntity,
		<Binding::Spec as ParabyzantineDataSpec>::MessageBuffer,
	>,
{
	type Binding = Binding;

	fn update_parabyzantine_hart(&mut self, data: &mut ParabyzantineWorld<Binding::Spec>) {
		for _ in 0..self.max_batch_size {
			match self.gossamer.try_recv_message::<Message>() {
				Ok(Some(message)) => {
					// Insert the message into the inferences
					data.message_inferences.insert(None, message)
				}
				Ok(None) => {
					// No message received
					break;
				}
				Err(e) => {
					// Insert the error into the inferences
					data.message_inferences.insert(None, e);
				}
			}
		}
	}
}
