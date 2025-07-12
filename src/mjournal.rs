use anyhow::Result;
use log::{error, info};
use std::{env, sync::Arc, time::Duration};
use systemd::journal;
use tokio::sync::mpsc::Sender;
use tokio_util::sync::CancellationToken;

use crate::{
    consts, discord, log_if_err, log_if_err_nest,
    target_log::{self, JournalConfig},
};

pub async fn observe_loop(
    target_yml: String,
    tx: Sender<(String)>,
    mut reload_rx: tokio::sync::watch::Receiver<()>,
) {
    let tx = Arc::new(tx);
    let mut handle: Option<tokio::task::JoinHandle<std::result::Result<(), anyhow::Error>>> = None;
    let mut cancel_token = CancellationToken::new();
    loop {
        let targets = match target_log::load_config(&target_yml) {
            Ok(t) => t,
            Err(e) => {
                error!("❌ 設定ファイル読み込み失敗: {}", e);
                tokio::time::sleep(Duration::from_secs(consts::SLEEP_SECS)).await;
                continue;
            }
        };

        if let Some(h) = handle.take() {
            info!("journal監視スレッド停止。。。");
            cancel_token.cancel();
            let joinerror = h.await;
            log_if_err_nest!(joinerror);
            cancel_token = CancellationToken::new();
        }

        let tx_clone = tx.clone();
        cancel_token = cancel_token.clone();
        let token = cancel_token.clone();

        handle = Some(tokio::task::spawn_blocking(move || -> Result<()> {
            let mut journal = journal::OpenOptions::default().open()?;

            journal.seek_tail()?;
            journal.previous()?;

            for target in targets.services.iter() {
                info!("Monitoring target: {:?}", &target);
                journal.match_add(target.get_unit_type().as_str(), target.get_name())?;
            }

            info!("🚩 jounarl監視を開始します");

            loop {
                if token.is_cancelled() {
                    info!("🛑 journal監視タスク: キャンセルされました");
                    return Ok(());
                }
                if let Some(entry) = journal.await_next_entry(None)? {
                    let msg = match entry.get(consts::FIELD_MESSAGE) {
                        Some(m) => m,
                        None => continue,
                    };

                    for target in targets.services.iter() {
                        let matched = match target.get_unit_type() {
                            target_log::JournalUnit::System => {
                                entry.get(consts::FIELD_SYSTEMD_UNIT).map(|v| v.as_str())
                                    == Some(target.get_name())
                            }
                            target_log::JournalUnit::User => {
                                entry
                                    .get(consts::FIELD_SYSTEMD_USER_UNIT)
                                    .map(|v| v.as_str())
                                    == Some(target.get_name())
                            }
                        };

                        if matched && target.keywords_contain(msg) {
                            let send_re = tx_clone.blocking_send(format!(
                                "🔔 [{}] {}",
                                target.get_name(),
                                msg
                            ));
                            log_if_err!(send_re);
                        }
                    }
                } else {
                    std::thread::sleep(std::time::Duration::from_millis(100));
                }
            }
        }));

        let re_changed = reload_rx.changed().await;
        log_if_err!(re_changed);
        info!("🔁 設定を再読み込みします");
    }
}
