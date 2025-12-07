from __future__ import annotations

import json
import logging
import re
from pathlib import Path
from typing import Dict, List

from .prompting import call_deepseek, load_prompt
from .c_parser import Feature

logger = logging.getLogger(__name__)


def generate_signatures(feature: Feature, api_key: str | None, prompt_file: Path, thinking: bool = False) -> List[str]:
    prompt = load_prompt(prompt_file)
    messages = [
        {"role": "system", "content": "You are an expert Rust engineer."},
        {"role": "user", "content": prompt.format(c_function=feature.code)},
    ]
    messages[-1]["content"] += "\n\n只返回 Rust 函数签名纯文本，不要解释，不要 Markdown 代码块。"
    resp = call_deepseek(messages, api_key=api_key, max_tokens=512, thinking=thinking)
    return _parse_signatures(resp, feature.name)


def translate_function(
    feature: Feature,
    target_sig: str,
    callee_sigs: List[str],
    static_hint: str,
    api_key: str | None,
    prompt_file: Path,
    max_tokens: int | None = None,
    thinking: bool = False,
) -> str:
    prompt = load_prompt(prompt_file)
    token_budget = max_tokens or _estimate_max_tokens(feature.code)
    user_content = prompt.format(
        target_sig=target_sig,
        c_function=feature.code,
        callee_sigs="\n".join(callee_sigs),
        static_hint=static_hint,
    )
    user_content += "\n\n只返回 Rust 代码，禁止 Markdown、解释或列表。"
    messages = [
        {"role": "system", "content": "Translate C to idiomatic safe Rust with minimal unsafe."},
        {"role": "user", "content": user_content},
    ]
    resp = call_deepseek(messages, api_key=api_key, max_tokens=token_budget, thinking=thinking)
    return _extract_rust_code(resp)


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


def _extract_rust_code(text: str) -> str:
    stripped = text.strip()
    if "```" in stripped:
        parts = stripped.split("```", 2)
        if len(parts) >= 3:
            block = parts[1 if parts[1].strip() else 2].strip()
            lines = block.splitlines()
            if lines and lines[0].strip().lower() in {"rust", "rs", "```rust"}:
                lines = lines[1:]
            return "\n".join(lines).strip()
    # Fallback: find the first likely Rust line and return from there
    lines = stripped.splitlines()
    for idx, line in enumerate(lines):
        if line.strip().startswith(("fn ", "pub ", "use ", "#!", "struct ", "enum ")):
            return "\n".join(lines[idx:]).strip()
    return stripped


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


def _estimate_max_tokens(code: str) -> int:
    # Rough char->token scaling; allow larger budgets for big bodies.
    approx = 512 + len(code) // 3
    return max(1500, min(8000, approx))


def _strip_code_fence(text: str) -> str:
    stripped = text.strip()
    if stripped.startswith("```") and "```" in stripped[3:]:
        inner = stripped.split("```", 2)[1]
        return inner.strip()
    return stripped


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
