pub mod common;
pub mod full_client;
pub mod local_cluster;

use clap::Parser;
use clap_markdown_ext::Markdown;

#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
pub enum Aegeri {
	/// Generate CLI documentation
	#[clap(subcommand)]
	Markdown(Markdown),
	/// Manage local cluster
	#[clap(subcommand)]
	LocalCluster(local_cluster::LocalCluster),
}

impl Aegeri {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		match self {
			Aegeri::Markdown(markdown) => {
				markdown.execute::<Self>().await?;
			}
			Aegeri::LocalCluster(local_cluster) => {
				local_cluster.execute().await?;
			}
		}

		Ok(())
	}
}
