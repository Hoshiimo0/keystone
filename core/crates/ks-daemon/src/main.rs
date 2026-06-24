//! ks-daemon -- Keystone LaunchDaemon Entry point
//!
//! アプリ起動監視(NSWorkspace は Swift 側, Daemon は受信側)
//! ブロックルールの評価
//! XPC 経由での UI との通信
//! LLM への問題生成リクエストの委譲

mod watcher; // Application blocking roup
mod xpc; // XPC Server
mod router; // DaemonRequest dispath

use anyhow::Result;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    // initialize log
    tracing_subscriber::fmt()
        .with_env_filter("ks-daemon=debug")
        .init();

    info!("Keystone daemon starting...");

    // TODO: ks-store 初期化
    // TODO: ks-scheduler 起動
    // TODO: XPC サーバ起動
    // TODO: watcher ループ起動
    
    //シグナル待機
    tokio::signal::ctrl_c().await?;
    info!("Keystone daemon stopping.");
    OK(())
}


