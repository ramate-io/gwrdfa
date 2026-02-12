/// A [Act] trait is a trait that acts on a value.
pub trait Act<A, D: Sized> {
	fn act(&mut self, action: A, data: &mut D);
}

/// A [Pair] trait indicates that a type can produce a disjoint pair.
pub trait Pair<A> {
	type Left;
	type Right;

	fn pair(&mut self) -> (&mut Self::Left, &mut Self::Right);
}

/// An [Invoke] trait invokes an action on self.
pub trait Invoke<A> {
	fn invoke(&mut self, action: A);
}

/// If you have a pair that acts, you can get an invoke for free.
impl<P, L, R, A> Invoke<A> for P
where
	P: Pair<A, Left = L, Right = R>,
	L: Act<A, R>,
{
	fn invoke(&mut self, action: A) {
		let (l, r) = self.pair();
		l.act(action, r);
	}
}
