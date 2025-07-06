use std::env;

use anyhow::Result;

mod consts;
mod discord;
mod logger;
mod mjournal;
pub mod target_log;
fn main() -> Result<()> {
    let _ = dotenvy::dotenv();
    logger::init();

    mjournal::observe_start()
}
