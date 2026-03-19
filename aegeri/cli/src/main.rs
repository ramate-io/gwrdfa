use aegeri::Aegeri;
use clap::*;
use dotenv::dotenv;
use std::sync::Once;

static LOG_INIT: Once = Once::new();

fn init_test_logger() {
	LOG_INIT.call_once(|| {
		let _ = env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
			.try_init();
	});
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
	// Load environment variables from .env file.
	dotenv().ok();

	// Initialize the logger
	init_test_logger();

	// Run the CLI.
	let aegeri = Aegeri::parse();
	aegeri.execute().await?;
	Ok(())
}
