//! ks-store -- SQLite 永続化レイヤ
//!
//! BlockRule の CRUD
//! クイズ履歴・解除ログ記録
//! DB マイグレーション

pub struct Store; // TODO: Connection フィールドを追加

impl Store {
    /// DB を開く
    pub fn open(_path: &str) -> anyhow::Result<Self> {
        todo!("rusqlite::Connection::open を実装")
    }
}
