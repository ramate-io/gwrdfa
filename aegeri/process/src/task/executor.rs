use aegeri_message::{Block, JoinSet, StateRoot, Transaction, TransactionSet, Transition};
use fuste_ecall_dispatcher::{EcallDispatcher, NoopDispatcher};
use fuste_exit::ExitStatus;
use fuste_exit_system::ExitSystem;
use fuste_interrupt_handler::{InterruptHandler, NoopEbreakDispatcher};
use fuste_lilbug::{LilBugComputer, LilBugSystem};
use fuste_riscv_core::{
	machine::{Machine, MachineError, MachineSystem},
	plugins::rv32i_computer::Rv32iComputer,
};
use fuste_riscv_elf::{Elf32Loader, ElfLoaderError};
use fuste_std_output_system::StdOutputSystem;
use std::ops::ControlFlow;

const TASK_MACHINE_MEMORY_SIZE: usize = 1024 * 1024 * 2;

#[derive(Debug, thiserror::Error)]
pub enum AegeriExecutionError {
	#[error("failed to load or parse ELF payload: {0}")]
	ElfLoader(#[from] ElfLoaderError),
	#[error("failed while running the Fuste machine: {0}")]
	Machine(#[from] MachineError),
}

struct EcallMachine {
	inner: InterruptHandler<
		TASK_MACHINE_MEMORY_SIZE,
		Rv32iComputer,
		EcallDispatcher<
			TASK_MACHINE_MEMORY_SIZE,
			ExitSystem<TASK_MACHINE_MEMORY_SIZE>,
			StdOutputSystem<TASK_MACHINE_MEMORY_SIZE>,
			NoopDispatcher<TASK_MACHINE_MEMORY_SIZE>,
			NoopDispatcher<TASK_MACHINE_MEMORY_SIZE>,
		>,
		NoopEbreakDispatcher<TASK_MACHINE_MEMORY_SIZE>,
	>,
}

impl MachineSystem<TASK_MACHINE_MEMORY_SIZE> for EcallMachine {
	fn tick(
		&mut self,
		machine: &mut Machine<TASK_MACHINE_MEMORY_SIZE>,
	) -> Result<ControlFlow<()>, MachineError> {
		self.inner.tick(machine)
	}
}

impl LilBugComputer<TASK_MACHINE_MEMORY_SIZE> for EcallMachine {
	fn exit_status(&self) -> ExitStatus {
		self.inner.ecall_dispatcher.exit_dispatcher.exit_status()
	}
}

/// Fuste-backed task executor for Aegeri transaction flows.
///
/// This is the concrete execution core for `AegeriTasks`:
/// - `Transaction::ElfScript` is loaded from bytes and executed on an ecall-enabled LilBug machine.
/// - Returns a `Value` for the block execution step.
pub struct AegeriExecutor {
	loader: Elf32Loader,
}

impl Default for AegeriExecutor {
	fn default() -> Self {
		Self::new()
	}
}

impl AegeriExecutor {
	pub fn new() -> Self {
		Self { loader: Elf32Loader::default() }
	}

	fn run_elf_script(&self, elf_bytes: &[u8]) -> Result<(), AegeriExecutionError> {
		let mut machine = Machine::<TASK_MACHINE_MEMORY_SIZE>::new();
		self.loader.load_elf(&mut machine, elf_bytes)?;

		let inner = InterruptHandler::<
			TASK_MACHINE_MEMORY_SIZE,
			Rv32iComputer,
			EcallDispatcher<
				TASK_MACHINE_MEMORY_SIZE,
				ExitSystem<TASK_MACHINE_MEMORY_SIZE>,
				StdOutputSystem<TASK_MACHINE_MEMORY_SIZE>,
				NoopDispatcher<TASK_MACHINE_MEMORY_SIZE>,
				NoopDispatcher<TASK_MACHINE_MEMORY_SIZE>,
			>,
			NoopEbreakDispatcher<TASK_MACHINE_MEMORY_SIZE>,
		> {
			inner: Rv32iComputer,
			ecall_dispatcher: EcallDispatcher {
				exit_dispatcher: ExitSystem::new(),
				write_dispatcher: StdOutputSystem::<TASK_MACHINE_MEMORY_SIZE>,
				open_channel_dispatcher: NoopDispatcher {},
				check_channel_dispatcher: NoopDispatcher {},
			},
			ebreak_dispatcher: NoopEbreakDispatcher {},
		};

		let ecall_machine = EcallMachine { inner };

		let mut lilbug = LilBugSystem {
			computer: ecall_machine,
			log_program_counter: false,
			log_instructions: false,
			log_registers: false,
			log_registers_at_end: false,
			log_exit_status: false,
		};
		machine.run(&mut lilbug)?;
		Ok(())
	}

	pub fn execute_block(&self, block: &Block) -> Result<Transition, AegeriExecutionError> {
		let mut transaction_set = TransactionSet::new();
		let mut join_set = JoinSet::new();
		for transaction in block.transactions() {
			transaction_set.add_id(transaction.id().clone());

			match transaction.payload() {
				Transaction::ElfScript(elf) => {
					self.run_elf_script(elf.as_bytes())?;
				}
				Transaction::Join => {
					join_set.add_member(transaction.public_key().clone());
				}
			}
		}

		// TODO: replace placeholder state-root computation with actual post-execution
		// state commitment once state plumbing is integrated.
		let state_root = StateRoot::new(Vec::new());

		Ok(Transition::new(transaction_set, state_root, join_set))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use aegeri_message::{Message, Nonce};
	use ml_dsa::{MlDsa44, SigningKey, B32};

	#[test]
	fn test_execute_joins() -> Result<(), anyhow::Error> {
		let executor = AegeriExecutor::new();
		let signer = SigningKey::<MlDsa44>::from_seed(&B32::from_iter(vec![7; 32]));
		let tx = Message::<Transaction>::try_new(&signer, Transaction::Join, Nonce::new(b"n0"))?;
		let verified_tx = tx.clone().into_verified()?;
		let block = Block::new(vec![verified_tx.clone()]);
		let value = executor.execute_block(&block)?;
		assert_eq!(value.block().ids().len(), 1);
		assert_eq!(value.block().iter_ids().next(), Some(verified_tx.id()));
		assert_eq!(value.state_root().as_bytes(), &[0; 0]);
		assert_eq!(value.join_set().members().len(), 1);
		assert_eq!(value.join_set().members()[0], *verified_tx.public_key());

		Ok(())
	}
}
