"""
generator; 問題生成ワーカ
    1. mlx_lm 
    2. anthoropic SDK
"""

from dataclasses import dataclass

@dataclass
class QuizRequest:
    subject:    str     # "math" | "physics" | "chemistry" | "literature" | ...
    difficulty: int     # 1–5
    fmt:        str     # "multiple_choice" | "open_ended"

@dataclass
class QuizResponse:
    question:    str
    format:      dict   # QuizFormat と対応する dict
    explanation: str

def generate(req: QuizRequest) -> QuizResponse:
    # TODO: mlx_lm を使って問題を生成
    # TODO: レスポンスを QuizResponse にパース
    raise NotImplementedError
