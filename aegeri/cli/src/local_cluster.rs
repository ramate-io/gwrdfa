pub mod quick_run;

use clap::Parser;

#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
pub enum LocalCluster {
	#[clap(subcommand)]
	QuickRun(quick_run::or_file::QuickRun),
}

impl LocalCluster {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		match self {
			LocalCluster::QuickRun(quick_run) => quick_run.execute().await,
		}
	}
}
