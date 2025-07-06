use log::{error, info};

const DISCORD_SUCCESS_CODE: &str = "204";

pub fn notify_discord(webhock_url: &str, msg: &str) {
    let client = reqwest::blocking::Client::new();
    let payload = serde_json::json!({"content":msg});

    let res = client.post(webhock_url).json(&payload).send();
    match res {
        Ok(resp) => {
            let status = resp.status();
            if status.as_str() == DISCORD_SUCCESS_CODE {
                info!("✅ Discord通知: 成功:{}", msg);
            } else {
                error!(
                    "❌ Discord通知失敗: {} 対象メッセージ: {}",
                    resp.status(),
                    msg
                );
            }
        }
        Err(e) => error!("❌ Discord通知失敗: {} 対象メッセージ: {}", e, msg),
    }
}
