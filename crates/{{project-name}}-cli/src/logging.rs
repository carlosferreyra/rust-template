//! Process-wide `tracing` subscriber.
//!
//! Library crates only emit spans/events with the `tracing` macros — they never
//! install a subscriber. The binary owns that decision so tests, examples, and
//! library consumers can install their own.

use tracing_subscriber::{EnvFilter, fmt};

/// Installs the global subscriber.
///
/// Reads filter directives from `RUST_LOG` (defaults to `info`). Emits JSON
/// when stderr is not a TTY (CI, log shippers), otherwise human-readable.
///
/// # Errors
///
/// Returns an error if a global subscriber is already installed.
pub(crate) fn init() -> anyhow::Result<()> {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let is_tty = std::io::IsTerminal::is_terminal(&std::io::stderr());

    let builder = fmt().with_env_filter(filter).with_writer(std::io::stderr);
    if is_tty {
        builder.try_init().map_err(|e| anyhow::anyhow!(e))
    } else {
        builder.json().try_init().map_err(|e| anyhow::anyhow!(e))
    }
}
