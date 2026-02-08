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
