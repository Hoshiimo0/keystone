# XPC Protocol — Keystone v0.1

Swift UI ↔ Rust daemon 間の IPC 仕様。
ks-ipc クレートの型定義が正本。この文書はその人間可読な補足。

## トランスポート

NSXPCConnection (mach service name: `com.keystone.daemon`)

## メッセージ形式

要求・応答ともに `serde_json` でシリアライズした 1 行 JSON。

## リクエスト一覧

| action             | 型                  | 説明                         |
|--------------------|---------------------|------------------------------|
| AddRule            | BlockRule           | ブロックルールを追加          |
| RemoveRule         | UUID                | ルールを削除                  |
| ListRules          | —                   | 全ルールを返す                |
| RequestQuiz        | subject, difficulty | 問題を生成して返す            |
| SubmitAnswer       | quiz_id, answer     | 回答を採点                    |
| UnlockApp          | bundle_id, quiz_id  | 正解時にアプリを一時解錠      |

## エラー

`DaemonResponse::Err(String)` で返す。UI 側は必ずハンドルすること。
