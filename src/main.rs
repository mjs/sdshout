mod config;
mod filter;
mod notify;
mod watch;

use eyre::Result;
use std::path::Path;

fn main() -> Result<()> {
    // XXX use directories crate
    // XXX fallback to default config
    let conf = config::load(Path::new("/home/menno/.config/sdshout/sdshout.toml"))?;
    println!("{:?}", conf);
    // XXX actually use the config
    watch::watch_units(filter::filter)
}
