/// The schedule for the boot step of the parabyzantine system.
///
/// Typically, this runs at the very start of the system and only once.
#[derive(Debug, Clone, Copy)]
pub struct ParabyzantineSystemBoot;

/// The prepare step of the parabyzantine system.
///
/// Typically, this is called at the top of the main loop of the system.
#[derive(Debug, Clone, Copy)]
pub struct ParabyzantineSystemPrepare;

/// The commit step of the parabyzantine system.
///
/// Typically, this is called at the bottom of the main loop of the system.
#[derive(Debug, Clone, Copy)]
pub struct ParabyzantineSystemCommit;
