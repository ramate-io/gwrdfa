use super::GossamerConfig;

#[derive(thiserror::Error, Debug)]
pub enum LocalClusterConfigError {
	#[error("Error allocating ports: {0}")]
	AllocatePortsError(String),
}

impl GossamerConfig {}
