from __future__ import annotations

import json
import logging
import re
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
    blocks = []
    seen: set[str] = set()
    for f in features:
        if f.name in seen:
            continue
        seen.add(f.name)
        body = translations.get(f.name, "").strip()
        if not body:
            if f.name == "main":
                body = "fn main() { /* TODO */ }"
            else:
                body = f"fn {f.name}() {{ unimplemented!(); }}" if f.kind == "function" else f"// missing translation for {f.name}"
        blocks.append(body)
    assembled = "\n\n".join(blocks)
    return _dedup_functions(assembled)


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


def _parse_signatures(resp: str, fallback_name: str) -> List[str]:
    text = _strip_code_fence(resp).strip()
    sigs: List[str] = []
    try:
        data = json.loads(text)
        if isinstance(data, dict):
            sigs = data.get("signatures", [])
    except Exception:  # noqa: BLE001
        pass
    if not sigs:
        for line in text.splitlines():
            line = line.strip()
            if line.startswith("fn "):
                sigs.append(line)
    if not sigs:
        sigs = [f"fn {fallback_name}() {{}}"]
    return sigs[:2]


def _dedup_functions(code: str) -> str:
    lines = code.splitlines()
    out: list[str] = []
    seen: set[str] = set()
    i = 0
    while i < len(lines):
        line = lines[i]
        m = re.match(r"\s*fn\s+([A-Za-z_]\w*)", line)
        if m:
            name = m.group(1)
            if name in seen:
                # skip this function body
                depth = line.count("{") - line.count("}")
                i += 1
                while i < len(lines) and depth > 0:
                    depth += lines[i].count("{") - lines[i].count("}")
                    i += 1
                continue
            seen.add(name)
        out.append(line)
        i += 1
    return "\n".join(out)
