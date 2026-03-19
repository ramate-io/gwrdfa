pub mod join;
pub mod leave;
pub mod send_elf;

use clap::Parser;

#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
pub enum FullClient {
	#[clap(subcommand)]
	Join(join::or_file::Join),
	#[clap(subcommand)]
	Leave(leave::or_file::Leave),
	#[clap(subcommand)]
	SendElf(send_elf::or_file::SendElf),
}

impl FullClient {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		match self {
			FullClient::Join(join) => join.execute().await,
			FullClient::Leave(leave) => leave.execute().await,
			FullClient::SendElf(send_elf) => send_elf.execute().await,
		}
	}
}
