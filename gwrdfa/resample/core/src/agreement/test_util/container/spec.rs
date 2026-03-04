use crate::agreement::Subcommittee;
use core::marker::PhantomData;

pub struct TestResampleParabyzantineSpec<Index: Eq, Value: Eq + 'static, Sub: Subcommittee<Value>> {
	__marker: PhantomData<(Index, Value, Sub)>,
}
