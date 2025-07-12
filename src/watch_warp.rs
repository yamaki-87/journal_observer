use std::sync::{Arc, atomic::AtomicBool};

use crate::log_if_err;
use log::{debug, error, info};
use notify::INotifyWatcher;
use notify::{EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::PathBuf;

pub struct WatchWarpper {
    watcher: INotifyWatcher,
}

impl Drop for WatchWarpper {
    fn drop(&mut self) {
        log::warn!("Watch Warpper is dropped");
    }
}

// 呼び出し側が保持できるように Result で返す
pub fn start_config_watcher(
    path: impl Into<PathBuf>,
    tx: tokio::sync::watch::Sender<()>,
) -> notify::Result<WatchWarpper> {
    let path = path.into();
    let parent = path.parent().expect("親ディレクトリが存在しません");
    let fname = path.file_name().unwrap().to_str().unwrap().to_string();
    let mut watcher = notify::recommended_watcher(move |res: notify::Result<notify::Event>| {
        let target_fname = fname.clone();
        match res {
            Ok(event) => {
                // デバッグ用：全部出力
                debug!("notify event: {:?}", event);

                match event.kind {
                    EventKind::Modify(_) => {
                        for p in event.paths {
                            if p.ends_with(target_fname.as_str()) {
                                info!("🌀 config.yml changed!");
                                let send_re = tx.send(());
                                log_if_err!(send_re);
                            }
                        }
                    }
                    _ => {}
                }
            }
            Err(e) => error!("watch error: {:?}", e),
        }
    })?;
    watcher.watch(parent, RecursiveMode::NonRecursive)?;
    info!("監視対象ファイル: {:?}", path);

    Ok(WatchWarpper { watcher: watcher })
}
