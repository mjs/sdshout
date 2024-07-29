mod notify;
mod watch;

use color_eyre::eyre::Result;

// TODO
// - github
// - figure out icons
// - report dead services on startup
// - release
// - rate limiting and/or aggregation
// - configurable delay
// - ignore some units

fn main() -> Result<()> {
    color_eyre::install()?;
    watch::watch_units(notify::notify)
}
