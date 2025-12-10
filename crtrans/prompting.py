import json
import logging
import os
from pathlib import Path
from typing import List, Dict, Any

import requests
from requests.adapters import HTTPAdapter
from urllib3.util.retry import Retry

import requests

logger = logging.getLogger(__name__)

DEEPSEEK_URL = "https://api.deepseek.com/v1/chat/completions"
DEFAULT_MODEL = "deepseek-reasoner"


def load_prompt(path: Path) -> str:
    return path.read_text(encoding="utf-8")


def call_deepseek(
    messages: List[Dict[str, str]],
    api_key: str | None = None,
    model: str = DEFAULT_MODEL,
    temperature: float = 0.1,
    max_tokens: int = 2048,
    max_retries: int = 3,
    timeout: float | tuple[float, float] = (20, 240),
    thinking: bool = False,
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
    if thinking:
        payload["thinking"] = {"type": "enabled"}

    headers = {
        "Content-Type": "application/json",
        "Authorization": f"Bearer {key}",
    }

    logger.info("Calling DeepSeek with %d tokens of messages", sum(len(m.get("content", "")) for m in messages))

    session = requests.Session()
    retry = Retry(
        total=max_retries,
        backoff_factor=2.0,
        status_forcelist=[500, 502, 503, 504],
        allowed_methods=["POST"],
        raise_on_status=False,
    )
    adapter = HTTPAdapter(max_retries=retry)
    session.mount("https://", adapter)
    session.mount("http://", adapter)

    try:
        resp = session.post(DEEPSEEK_URL, headers=headers, data=json.dumps(payload), timeout=timeout)
        resp.raise_for_status()
        data = resp.json()
        return data["choices"][0]["message"]["content"]
    except requests.exceptions.RequestException as exc:
        logger.error("DeepSeek call failed after retries: %s", exc)
        raise RuntimeError("DeepSeek request failed") from exc
    except Exception as exc:  # noqa: BLE001
        logger.error("DeepSeek response parse failed: %s", resp.text if 'resp' in locals() else "<no response>")
        raise RuntimeError("Unexpected DeepSeek response") from exc
    resp = requests.post(DEEPSEEK_URL, headers=headers, data=json.dumps(payload), timeout=120)
    resp.raise_for_status()
    data = resp.json()
    try:
        return data["choices"][0]["message"]["content"]
    except Exception as exc:  # noqa: BLE001
        logger.error("DeepSeek response parse failed: %s", data)
        raise RuntimeError("Unexpected DeepSeek response") from exc
