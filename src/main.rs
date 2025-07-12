use std::env;

use anyhow::Result;

mod consts;
mod discord;
mod logger;
mod macros;
mod mjournal;
pub mod target_log;
mod watch_warp;

#[tokio::main]
async fn main() -> Result<()> {
    let _ = dotenvy::dotenv();
    logger::init();

    let target_yml =
        env::var(consts::ENV_TARGET_YML).expect("環境変数 TARGET_YML が設定されていません");
    let (log_tx, mut log_rx) = tokio::sync::mpsc::channel(100);
    let (reload_tx, mut reload_rx) = tokio::sync::watch::channel(());
    //let (relaod_tx, mut reload_rx) = tokio::sync::broadcast::channel(16);

    //target_log::watch_config_file(target_yml.clone(), reload_tx);
    let _watcher = watch_warp::start_config_watcher(target_yml.clone(), reload_tx)?;
    tokio::spawn(mjournal::observe_loop(target_yml, log_tx, reload_rx));

    let webhock =
        env::var(consts::ENV_WEBHOCK).expect("環境変数 DISCORD_WEBHOOK が設定されていません");
    while let Some(msg) = log_rx.recv().await {
        discord::notify_discord(&webhock, &msg).await;
    }
    Ok(())
}
