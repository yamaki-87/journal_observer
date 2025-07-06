use anyhow::Result;
use log::info;
use std::env;
use systemd::journal;

use crate::{consts, discord, target_log};

pub fn observe_start() -> Result<()> {
    let webhock =
        env::var(consts::ENV_WEBHOCK).expect("環境変数 DISCORD_WEBHOOK が設定されていません");
    let mut journal = journal::OpenOptions::default().open()?;

    journal.seek_tail()?;
    journal.previous()?;

    let targets = target_log::load_config(
        env::var(consts::ENV_TARGET_YML)
            .expect("環境変数 TARGET_YML が設定されていません")
            .as_str(),
    )?;
    for target in targets.services.iter() {
        info!("Monitoring target: {:?}", &target);
        journal.match_add(target.get_unit_type().as_str(), target.get_name())?;
    }

    info!("jounarl監視を開始します");

    loop {
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
                    discord::notify_discord(
                        &webhock,
                        &format!("🔔 [{}] {}", target.get_name(), msg),
                    );
                }
            }
        } else {
            // If no new entry, sleep for a while to avoid busy waiting
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }
}
