pub mod join;
pub mod leave;
pub mod send_elf;

use clap::Parser;

#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
pub enum LocalCluster {}

impl LocalCluster {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		match self {
			_ => Ok(()),
		}
	}
}
