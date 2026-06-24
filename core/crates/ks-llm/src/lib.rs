//! ks-llm -- LLM バックエンド抽象レイヤ
//!
//! LlmBackend トレイとで local/API を統一インタフェース化
//! LocalBackend: Python サブプロセスを stdin/stdout JSON で制御
//! ClaudeBackend: Anthropic API を呼び出す

use anyhow::Result;
use async_trait::async_trait;
use ks_ipc::{Quiz, Subject};

// -- trait --
#[async_trait]
pub trait LlmBackend: Send + Sync {
    /// 問題を生成して返す
    async fn generate(&self, subject: Subject, difficulty: u8) -> Result<Quiz>;

    /// 記述式回答を採点し (correct, score, feedback) を返す
    async fn evaluate(
        &self,
        question: &str,
        correct:  &str,
        answer:   &str,
    ) -> Result<(bool, f32, String)>;
}

// -- LocalBackend --
pub struct LocalBackend;    // TODO: Child プロセスハンドルを保持

impl LocalBackend {
    pub async fn try_new() -> Result<Self> {
        todo!("llm/engine/server.py を tokio::process::Command で起動")
    }
}

#[async_trait]
impl LlmBackend for LocalBackend {
    async fn generate(&self, _subject: Subject, _difficulty: u8) -> Result<Quiz> {
        todo!("Writing JSON requests, and Parsing Quiz from stdout")
    }

    async fn evaluate(&self, _q: &str, _c: &str, _a: &str) -> Result<(bool, f32, String)> {
        todo!()
    }
}

// ClaudeBackend --
pub struct ClaudeBackend;   // TODO: reqwest::Client + api_key を保持

impl ClaudeBackend {
    pub fn new(_api_key: &str) -> Self { Self }
}

#[async_trait]
impl LlmBackend for ClaudeBackend {
    async fn generate(&self, _subject: Subject, _difficulty: u8) -> Result<Quiz> {
        todo!("send prompt to POST /v1/messages ")
    }

    async fn evaluate(&self, _q: &str, _c: &str, _a: &str) -> Result<(bool, f32, String)> {
        todo!()
    }
}

// -- factory --
/// LocalBackend
pub async fn build_backend(claude_api_key: &str) -> Box<dyn LlmBackend> {
    match LocalBackend::try_new().await {
        Ok(b)  => Box::new(b),
        Err(e) => {
            tracing::warn!("Local LLM unavailable ({e}), falling back to Claude API");
            Box::new(ClaudeBackend::new(claude_api_key))
        }
    }
}
