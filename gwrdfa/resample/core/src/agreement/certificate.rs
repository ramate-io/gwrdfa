use super::Subcommittee;
use parabyzantine::NoOp;

pub trait Certificate<Index: Eq, Value: Eq, Sender: Eq>: Eq + Sized {
	/// The index of the message.
	fn index(&self) -> Index;

	/// The value of the message.
	fn value(&self) -> Value;

	/// The sender of the message.
	fn sender(&self) -> Sender;
}

pub trait CertificateSet<
	Index: Eq,
	Value: Eq,
	Sender: Eq,
	Item: Certificate<Index, Value, Sender>,
	Sub: Subcommittee<Sender>,
>: Eq + Sized
{
	fn contains(&self, item: &Item) -> bool;

	fn insert(&mut self, item: Item);

	fn remove(&mut self, item: Item);

	fn partial_subcommittees_for_index<'a>(
		&'a self,
		index: &Index,
	) -> impl Iterator<Item = (&'a Sub, &'a Value)> + 'a
	where
		Self: 'a,
		Sub: 'a,
		Value: 'a;

	fn partial_subcommittee_for_value<'a>(
		&'a self,
		index: &Index,
		value: &Value,
	) -> Option<(&'a Sub, &'a Value)>
	where
		Self: 'a,
		Sub: 'a,
		Value: 'a;
}

/// A [Certificate] for the [NoOp] struct.
impl Certificate<NoOp, NoOp, NoOp> for NoOp {
	fn index(&self) -> NoOp {
		NoOp
	}
	fn value(&self) -> NoOp {
		NoOp
	}
	fn sender(&self) -> NoOp {
		NoOp
	}
}

/// A [CertificateSet] for the [NoOp] struct.
impl CertificateSet<NoOp, NoOp, NoOp, NoOp, NoOp> for NoOp {
	fn contains(&self, _item: &NoOp) -> bool {
		false
	}
	fn insert(&mut self, _item: NoOp) {}
	fn remove(&mut self, _item: NoOp) {}
	fn partial_subcommittees_for_index<'a>(
		&'a self,
		_index: &NoOp,
	) -> impl Iterator<Item = (&'a NoOp, &'a NoOp)> + 'a
	where
		Self: 'a,
		NoOp: 'a,
	{
		[].into_iter()
	}

	fn partial_subcommittee_for_value<'a>(
		&'a self,
		_index: &NoOp,
		_value: &NoOp,
	) -> Option<(&'a NoOp, &'a NoOp)>
	where
		Self: 'a,
		NoOp: 'a,
	{
		None
	}
}
