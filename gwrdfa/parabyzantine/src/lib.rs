#![no_std]

pub mod buffer;
pub mod parabyzantine;

pub trait Container: Sized {
	fn member<M: Member<Self>>(&self) -> &M {
		M::as_ref(self)
	}

	fn member_mut<M: Member<Self>>(&mut self) -> &mut M {
		M::as_mut(self)
	}
}

pub trait Member<C> {
	fn as_ref(of: &C) -> &Self;

	fn as_mut(of: &mut C) -> &mut Self;
}

impl<C> Container for C where C: Sized {}

pub trait Factory: Sized {
	fn produce<P: Product<Self>>(&self) -> P {
		P::derive(self)
	}
}

pub trait Product<F> {
	fn derive(from: &F) -> Self;
}

impl<F> Factory for F where F: Sized {}

pub trait Scene: Sized {
	fn view<'a, V>(&'a self) -> V
	where
		V: View<'a, Self>,
	{
		V::view(self)
	}
}

pub trait View<'a, F> {
	fn view(from: &'a F) -> Self;
}

impl<F> Scene for F where F: Sized {}

pub trait Host: Sized {
	fn visit<V: Visitor<Self>>(&mut self, visitor: &mut V) {
		visitor.visit(self);
	}
}

pub trait Visitor<H> {
	fn visit(&mut self, host: &mut H);
}

impl<H> Host for H where H: Sized {}

pub trait Cocontainer: Sized {
	fn comember<R, Co: Comember<R, Self>>(&self) -> &Co {
		Co::as_coref(self)
	}

	fn comember_mut<R, Co: Comember<R, Self>>(&mut self) -> &mut Co {
		Co::as_comut(self)
	}
}

/// Comember is a trait that allows the extraction of a member from a container via a related type.
///
/// This is often useful when you want to define some higher-level protocol,
/// which varies with underlying types.
///
/// "Grab be whatever the underlying type mapped to the well known comember type is."
pub trait Comember<R, C> {
	fn as_coref(of: &C) -> &Self;

	fn as_comut(of: &mut C) -> &mut Self;
}

impl<C> Cocontainer for C where C: Sized {}

pub trait Tasker: Sized {
	fn task<'a, Referrant, ScheduleVisitor, Container, Data>(&mut self, container: &'a Container)
	where
		ScheduleVisitor: Comember<Referrant, Self> + Visitor<Data> + Sized,
		Data: View<'a, Container>;
}

impl<S> Tasker for S
where
	S: Sized,
{
	fn task<'a, Referrant, ScheduleVisitor, Container, Data>(&mut self, container: &'a Container)
	where
		ScheduleVisitor: Comember<Referrant, Self> + Visitor<Data> + Sized,
		Data: View<'a, Container>,
	{
		let schedule_visitor = self.comember_mut::<Referrant, ScheduleVisitor>();
		let mut data = container.view::<'a, Data>();
		schedule_visitor.visit(&mut data);
	}
}
