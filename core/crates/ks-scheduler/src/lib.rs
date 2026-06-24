//! ks-scheduler -- ブロックルールの時刻評価,タイマー管理
//!
//! Schedule を評価しブロック中か返却
//! 次のブロック開始/終了イベントを tokio::time で発火
//! ks-daemon の watcher からポーリング

pub struct Scheduler; //TODO: フィールドを追加

impl Scheduler {
    pub fn new() -> Self { Self }

    /// bundle_id が現在のブロック対象か
    pub fn is_blocked(&self, _bundle_id: &str)-> bool {
        todo!("Schedule logic of rublic")
    }
}

