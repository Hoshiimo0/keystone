//! ks-quiz -- クイズモデル・採点ロジック
//!
//! LLM レスポンスを Quiz 型にパース
//! ユーザ回答を採点
//! 難易度・科目別バリエーション

use ks_ipc::{Quiz, QuizFormat};

pub struct QuizEngine;

impl QuizEngine {
    pub fn new() -> Self { Self }

    /// ユーザー回答を採点し 0.0–1.0 のスコアを返す
    pub fn evaluate(&self, quiz: &Quiz, user_answer: &str) -> f32 {
        match &quiz.format {
            QuizFormat::MultipleChoice { correct_index, choices } => {
                todo!("Checking...")
            }
            QuizFormat::OpenEnded { correct_answer } => {
                todo!("Calculating...")
            }
        }
    }
}
