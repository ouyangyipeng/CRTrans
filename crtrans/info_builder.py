from __future__ import annotations

import json
import logging
import subprocess
from subprocess import CalledProcessError
from pathlib import Path
from typing import List, Tuple

from .prompting import call_deepseek, load_prompt

logger = logging.getLogger(__name__)


def compile_c(c_path: Path, output: Path) -> None:
    cmd = ["gcc", "-O0", str(c_path), "-o", str(output)]
    logger.info("Compiling C: %s", " ".join(cmd))
    try:
        subprocess.run(cmd, check=True, capture_output=True, text=True)
    except CalledProcessError as exc:  # noqa: BLE001
        msg = exc.stderr or exc.stdout or str(exc)
        raise RuntimeError(f"C compilation failed: {msg}") from exc


def _run_program(bin_path: Path, input_text: str, timeout: int = 5) -> Tuple[int, str, str]:
    proc = subprocess.run(
        [str(bin_path)],
        input=input_text.encode(),
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        timeout=timeout,
    )
    return proc.returncode, proc.stdout.decode(), proc.stderr.decode()


def ask_llm_for_info(c_source: str, api_key: str | None, prompt_file: Path, thinking: bool = False) -> Tuple[str, List[str], str]:
    prompt = load_prompt(prompt_file)
    messages = [
        {"role": "system", "content": "You are a senior systems engineer."},
        {
            "role": "user",
            "content": prompt.format(c_source=c_source),
        },
    ]
    resp = call_deepseek(messages, api_key=api_key, max_tokens=1024, thinking=thinking)
    # Expect JSON lines with description + samples array + notes
    parsed = _parse_info_json(resp)
    if parsed:
        return parsed

    # Deterministic fallback: simple description and empty sample
    logger.info("LLM info parse failed; using deterministic fallback")
    return "Auto-generated description", [""], ""


def _parse_info_json(resp: str) -> Tuple[str, List[str], str] | None:
    text = resp.strip()
    if text.startswith("```") and text.endswith("```"):
        text = "\n".join(text.splitlines()[1:-1])
    try:
        data = json.loads(text)
        desc = data.get("description", "")
        samples = data.get("samples", [])
        notes = data.get("notes", "")
        if not isinstance(samples, list):
            samples = []
        if not samples:
            samples = [""]
        return desc, samples[:4], notes
    except Exception:  # noqa: BLE001
        return None


def build_info(
    c_path: Path, work_dir: Path, prompt_dir: Path, api_key: str | None, thinking: bool = False
) -> tuple[Path, list[dict[str, str | int]]]:
    c_source = c_path.read_text(encoding="utf-8")
    bin_path = work_dir / "c_binary"
    work_dir.mkdir(parents=True, exist_ok=True)

    compile_c(c_path, bin_path)
    prompt_file = prompt_dir / "info_prompt.txt"
    desc, samples, notes = ask_llm_for_info(c_source, api_key, prompt_file, thinking=thinking)

    # Fallback sample when LLM does not provide one.
    if samples == [""]:
        samples = ["5\n5 4 3 2 1\n"]

    outputs = []
    for idx, sample in enumerate(samples, 1):
        rc, out, err = _run_program(bin_path, sample)
        if rc != 0:
            raise RuntimeError(f"C program failed on sample {idx} (rc={rc}): {err or out}")
        outputs.append({"input": sample, "returncode": rc, "stdout": out, "stderr": err})

    info_path = work_dir / "info.md"
    lines = [
        f"# Info for {c_path.name}",
        "",
        "## Description",
        desc or "(auto-generated)",
        "",
        "## Samples",
    ]
    for i, sample in enumerate(outputs, 1):
        lines.append(f"### Sample {i}")
        lines.append("Input:")
        lines.append("````")
        lines.append(sample["input"])
        lines.append("````")
        lines.append("Output:")
        lines.append("````")
        lines.append(sample["stdout"])
        lines.append("````")
        if sample["stderr"]:
            lines.append("Stderr:")
            lines.append("````")
            lines.append(sample["stderr"])
            lines.append("````")
        lines.append("Return code: %d" % sample["returncode"])
        lines.append("")
    if notes:
        lines.append("## Notes")
        lines.append(notes)
    info_path.write_text("\n".join(lines), encoding="utf-8")
    logger.info("Wrote info.md to %s", info_path)
    return info_path, outputs
