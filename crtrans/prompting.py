import json
import logging
import os
from pathlib import Path
from typing import List, Dict, Any

import requests

logger = logging.getLogger(__name__)

DEEPSEEK_URL = "https://api.deepseek.com/v1/chat/completions"
DEFAULT_MODEL = "deepseek-chat"


def load_prompt(path: Path) -> str:
    return path.read_text(encoding="utf-8")


def call_deepseek(
    messages: List[Dict[str, str]],
    api_key: str | None = None,
    model: str = DEFAULT_MODEL,
    temperature: float = 0.2,
    max_tokens: int = 2048,
) -> str:
    key = api_key or os.getenv("DEEPSEEK_API_KEY")
    if not key:
        raise RuntimeError("DeepSeek API key missing; set DEEPSEEK_API_KEY or pass --api-key")

    payload: Dict[str, Any] = {
        "model": model,
        "messages": messages,
        "temperature": temperature,
        "max_tokens": max_tokens,
    }

    headers = {
        "Content-Type": "application/json",
        "Authorization": f"Bearer {key}",
    }

    logger.info("Calling DeepSeek with %d tokens of messages", sum(len(m.get("content", "")) for m in messages))
    resp = requests.post(DEEPSEEK_URL, headers=headers, data=json.dumps(payload), timeout=120)
    resp.raise_for_status()
    data = resp.json()
    try:
        return data["choices"][0]["message"]["content"]
    except Exception as exc:  # noqa: BLE001
        logger.error("DeepSeek response parse failed: %s", data)
        raise RuntimeError("Unexpected DeepSeek response") from exc
