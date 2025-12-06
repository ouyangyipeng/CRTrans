from __future__ import annotations

import json
import logging
from pathlib import Path
from typing import Dict, List

from .prompting import call_deepseek, load_prompt
from .c_parser import Feature

logger = logging.getLogger(__name__)


def generate_signatures(feature: Feature, api_key: str | None, prompt_file: Path) -> List[str]:
    prompt = load_prompt(prompt_file)
    messages = [
        {"role": "system", "content": "You are an expert Rust engineer."},
        {"role": "user", "content": prompt.format(c_function=feature.code)},
    ]
    resp = call_deepseek(messages, api_key=api_key, max_tokens=512)
    try:
        data = json.loads(resp)
        sigs = data.get("signatures", [])
        return sigs[:2] if sigs else []
    except Exception:  # noqa: BLE001
        logger.warning("Signature parse failed, fallback to raw text")
        return resp.splitlines()[:2]


def translate_function(
    feature: Feature,
    target_sig: str,
    callee_sigs: List[str],
    static_hint: str,
    api_key: str | None,
    prompt_file: Path,
) -> str:
    prompt = load_prompt(prompt_file)
    messages = [
        {"role": "system", "content": "Translate C to idiomatic safe Rust with minimal unsafe."},
        {
            "role": "user",
            "content": prompt.format(
                target_sig=target_sig,
                c_function=feature.code,
                callee_sigs="\n".join(callee_sigs),
                static_hint=static_hint,
            ),
        },
    ]
    resp = call_deepseek(messages, api_key=api_key, max_tokens=2048)
    return _strip_code_fence(resp)


def assemble_rust(features: List[Feature], translations: Dict[str, str]) -> str:
    blocks = [translations.get(f.name, f"// TODO missing translation for {f.name}") for f in features]
    return "\n\n".join(blocks)


def _strip_code_fence(text: str) -> str:
    if text.strip().startswith("```"):
        lines = text.strip().splitlines()
        # drop first and last fence lines if present
        if lines[0].startswith("```"):
            lines = lines[1:]
        if lines and lines[-1].startswith("```"):
            lines = lines[:-1]
        return "\n".join(lines)
    return text
