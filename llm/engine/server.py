"""
server.py; Rust からの stdin/stdout JSON プロトコルサーバ
Protocol
    - Request; one line JSON -> {"action":"generate", "subject":"math", "difficulty":3}
        or {"action":"evaluate","question":...,"correct":...,"answer":...}
    - Responsel one line JSON -> {"ok":true, "data":{...}}
        or {"ok":false, "error":"..."}
"""

import sys
import json

def handle(request: dict) -> dict:
    action = request.get("action")

    if action == "generate":
        # TODO: generator.py を呼ぶ
        raise NotImplementedError("generate")

    elif action == "evaluate":
        # TODO: evaluator.py を呼ぶ
        raise NotImplementedError("evaluate")

    else:
        raise ValueError(f"unknown action: {action}")


def main() -> None:
    for line in sys.stdin:
        line = line.strip()
        if not line:
            continue
        try:
            req = json.loads(line)
            data = handle(req)
            print(json.dumps({"ok": True, "data": data}), flush=True)
        except Exception as e:
            print(json.dumps({"ok": False, "error": str(e)}), flush=True)

if __name__ == "__main__":
    main()
