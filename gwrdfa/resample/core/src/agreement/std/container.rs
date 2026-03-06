//! Container-backed adapters used by `agreement::std`.
//!
//! These modules define:
//! - concrete container shapes (`agreement`, `certificate`),
//! - their delta types (`AgreementDelta`, `CertificateDelta`),
//! - an adapter implementing `ParabyzantineAgreementData` over container buffers
//!   (`parabyzantine_data`).

pub mod agreement;
pub use agreement::*;

pub mod certificate;
pub use certificate::*;

pub mod parabyzantine_data;
pub use parabyzantine_data::*;
