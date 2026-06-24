"""
evaluator; 採点ワーカ
"""

from dataclasses import dataclass

@dataclass
class EvalRequest:
    question:    str
    correct:     str
    user_answer: str


@dataclass
class EvalResult:
    correct:  bool
    score:    float   # 0.0–1.0
    feedback: str


def evaluate(req: EvalRequest) -> EvalResult:
    # TODO: LLM に採点プロンプトを投げて EvalResult を返す
    raise NotImplementedError
