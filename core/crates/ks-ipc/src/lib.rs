//! ks-ipc -- XPC/IPC 用のメッセージ型定義 (Swift <-> Rust)
//!
//! This crate doesn't have buisness logic.
//! only type and static number
//! synclonize XPC_PROTOCOL.md

use serde::{Deserialize, Serialize};

// -- block rule --

/// ブロック対象アプリの拡張子(bundle ID)
pub type BundleId = String;

/// ブロックスケジュール
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockRule {
    pub id: uuid_placeholder,
    pub bundle_id: BundleId,
    pub schedule: Schedule,
    pub enabled: bool,
}

/// ブロック自国の定義
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Schedule {
    /// 指定時日時から指定時間ブロック
    Duration { start_iso8601: String, hours: u32},
    /// 毎日の時間帯
    Daily { from_hhmm: String, to_hhmm: String },
    /// cron
    Cron (String),
}

// -- Quiz --
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Subject {
    Math,
    Physics,
    Chemistry,
    Literature,
    History,
    ComputerScience,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuizFormat {
    /// 選択式
    MultipleChoise { choices: Vec<String>, correct_index: usize},
    /// 記述式
    OpenEnded { correct_answer: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quiz {
    pub id: uuid_placeholder,
    pub subject: Subject,
    pub difficulty: u8,
    pub question: String,
    pub format: QuizFormat,
    pub explanation: String,
}

// -- IPC message --
/// UI -> Daemon 
#[derive(Debug, Serialize, Deserialize)]
pub enum DaemonRequest {
    AddRule(BlockRule),
    RemoveRule(uuid_placeholder),
    ListRules,
    RequestQuiz { subject: Subject, difficulty: u8 },
    SubmitAnswer { quiz_id: uuid_placeholder, answer: String },
    UnlockApp { bundle_id: BundleId, quiz_id: uuid_placeholder },
}

/// Daemon -> UI
#[derive(Debug, Serialize, Deserialize)]
pub enum DaemonResponse {
    Rules(Vec<BlockRule>),
    QuizReady(Quiz),
    EvalResult { correct: bool, score: f32, feedback: String },
    Ok,
    Err(String),
}

