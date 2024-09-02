mod config;
mod notify;
mod watch;

use eyre::Result;

fn main() -> Result<()> {
    watch::watch_units(notify::notify)
}
