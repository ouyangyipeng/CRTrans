#!/usr/bin/env python3
from __future__ import annotations

import argparse
import logging
from pathlib import Path
from typing import Dict, List

from crtrans.logging_setup import setup_logging
from crtrans.c2rust_wrapper import write_compile_commands, run_c2rust
from crtrans.c_parser import CFeatureExtractor, topo_sort, Feature
from crtrans.info_builder import build_info
from crtrans.prompting import load_prompt, call_deepseek
from crtrans.translator import generate_signatures, translate_function, assemble_rust
from crtrans.rust_checker import compile_rust
from crtrans.runner import run_binary, compare_outputs


logger = logging.getLogger(__name__)


def find_c_file(c_arg: str | None) -> Path:
    if c_arg:
        return Path(c_arg).resolve()
    c_dir = Path("C")
    candidates = list(c_dir.glob("*.c"))
    if len(candidates) != 1:
        raise RuntimeError("Provide --c-file or keep exactly one .c file in C/")
    return candidates[0].resolve()


def pick_static_hint(output_dir: Path) -> str:
    src_dir = output_dir / "src"
    if not src_dir.exists():
        return ""
    rs_files = list(src_dir.glob("*.rs"))
    if not rs_files:
        return ""
    return rs_files[0].read_text(encoding="utf-8")


def compute_iteration_budget(c_path: Path, features: List[Feature], cli_max: int) -> int:
    """Derive iteration budget from file length and chunk count, honoring CLI minimum."""
    try:
        text = c_path.read_text(encoding="utf-8", errors="ignore")
        line_count = text.count("\n") + 1
    except Exception:  # noqa: BLE001
        line_count = 0
    chunk_count = len(features)

    baseline = max(cli_max, 10)
    size_factor = line_count // 120  # grow faster for long files
    chunk_factor = max(1, chunk_count // 3)  # bump for many chunks

    budget = baseline + size_factor + chunk_factor
    budget = min(budget, 50)

    logger.info(
        "Iteration budget set to %d (lines=%d, chunks=%d, cli_min=%d)",
        budget,
        line_count,
        chunk_count,
        cli_max,
    )
    return budget


def write_final_report(c_path: Path, rust_path: Path, prompt_dir: Path, api_key: str | None, work_dir: Path) -> None:
    prompt_file = prompt_dir / "final_judge_prompt.txt"
    prompt = load_prompt(prompt_file)
    c_code = c_path.read_text(encoding="utf-8")
    rust_code = rust_path.read_text(encoding="utf-8")
    messages = [
        {"role": "system", "content": "You are a precise C/Rust equivalence reviewer."},
        {
            "role": "user",
            "content": prompt.format(c_name=c_path.name, c_code=c_code, rust_name=rust_path.name, rust_code=rust_code),
        },
    ]
    resp = call_deepseek(messages, api_key=api_key, max_tokens=2048)
    report_path = work_dir / f"report_{c_path.stem}.md"
    work_dir.mkdir(parents=True, exist_ok=True)
    report_path.write_text(resp, encoding="utf-8")
    logger.info("Wrote final equivalence report to %s", report_path)


def main() -> None:
    parser = argparse.ArgumentParser(description="C to Rust transpilation orchestrator")
    parser.add_argument("--c-file", help="Path to C file", default=None)
    parser.add_argument("--api-key", help="DeepSeek API key", default=None)
    parser.add_argument("--work-dir", default="temp", help="Working directory for intermediates")
    parser.add_argument("--rust-out", default="rust/translated.rs", help="Final rust output file")
    parser.add_argument("--max-fix-iters", type=int, default=10, help="Max rustc+LLM fix iterations (syntax and output loops combined)")
    args = parser.parse_args()

    work_dir = Path(args.work_dir)
    log_dir = work_dir / "logs"
    setup_logging(log_dir)

    c_path = find_c_file(args.c_file)
    logger.info("Using C file: %s", c_path)

    prompt_dir = Path("prompt")
    info_path, c_outputs = build_info(c_path, work_dir, prompt_dir, args.api_key)
    samples = [o["input"] for o in c_outputs]

    compile_commands = work_dir / "compile_commands.json"
    write_compile_commands(c_path, compile_commands)
    c2rust_out = work_dir / "c2rust"
    try:
        run_c2rust(compile_commands, c2rust_out)
    except Exception as exc:  # noqa: BLE001
        logger.warning("c2rust failed: %s", exc)
    static_hint = pick_static_hint(c2rust_out)

    extractor = CFeatureExtractor(c_path)
    features = topo_sort(extractor.parse())

    iteration_budget = compute_iteration_budget(c_path, features, args.max_fix_iters)

    translations: Dict[str, str] = {}
    chosen_sigs: Dict[str, str] = {}

    sig_prompt = prompt_dir / "signature_prompt.txt"
    translate_prompt = prompt_dir / "translate_prompt.txt"

    for feat in features:
        if feat.kind != "function":
            translations[feat.name] = feat.code
            continue
        if feat.name == "main":
            target_sig = "fn main()"
        else:
            sigs = generate_signatures(feat, args.api_key, sig_prompt)
            target_sig = sigs[0] if sigs else f"fn {feat.name}() {{ unimplemented!() }}"
        chosen_sigs[feat.name] = target_sig
        callees = [chosen_sigs.get(dep, dep) for dep in feat.deps if dep in chosen_sigs]
        rust_code = translate_function(feat, target_sig, callees, static_hint, args.api_key, translate_prompt)
        if not rust_code.strip():
            rust_code = target_sig + " { unimplemented!(); }"
        translations[feat.name] = rust_code

    assembled = assemble_rust(features, translations)
    rust_out_path = Path(args.rust_out)
    rust_out_path.parent.mkdir(parents=True, exist_ok=True)
    rust_out_path.write_text(assembled, encoding="utf-8")
    logger.info("Wrote assembled Rust to %s", rust_out_path)

    ok, stderr = compile_rust(rust_out_path)
    fix_prompt = prompt_dir / "fix_prompt.txt"
    compare_fix_prompt = prompt_dir / "compare_fix_prompt.txt"
    iter_idx = 0
    while not ok and iter_idx < iteration_budget:
        iter_idx += 1
        logger.warning("rustc failed; invoking LLM fix iteration %d", iter_idx)
        current = rust_out_path.read_text(encoding="utf-8")
        fixed = translate_function(  # reuse helper to call LLM with fix prompt
            Feature("file", "function", current, set()),
            target_sig="",
            callee_sigs=[],
            static_hint=stderr,
            api_key=args.api_key,
            prompt_file=fix_prompt,
            max_tokens=6000,
        )
        rust_out_path.write_text(fixed, encoding="utf-8")
        ok, stderr = compile_rust(rust_out_path)

    if not ok:
        logger.error("Rust compilation failed after fixes; inspect %s", rust_out_path)
        return

    rust_bin = rust_out_path.with_suffix("")
    r_outputs = run_binary(rust_bin, samples)
    diffs = compare_outputs(c_outputs, r_outputs)
    iter_cmp = 0
    while diffs and (iter_idx + iter_cmp) < iteration_budget:
        logger.info("Output mismatches detected; invoking LLM fix iteration %d", iter_cmp + 1)
        iter_cmp += 1
        current = rust_out_path.read_text(encoding="utf-8")
        diff_text = "\n".join(diffs)
        fixed = translate_function(
            Feature("file", "function", current, set()),
            target_sig="",
            callee_sigs=[],
            static_hint=diff_text,
            api_key=args.api_key,
            prompt_file=compare_fix_prompt,
            max_tokens=6000,
        )
        rust_out_path.write_text(fixed, encoding="utf-8")
        ok, stderr = compile_rust(rust_out_path)
        if not ok:
            logger.info("Fix iteration produced compile errors; attempting next iteration (syntax takes priority)")
            continue
        rust_bin = rust_out_path.with_suffix("")
        r_outputs = run_binary(rust_bin, samples)
        diffs = compare_outputs(c_outputs, r_outputs)

    if diffs:
        logger.warning("Output mismatches:\n%s", "\n".join(diffs))
    else:
        logger.info("Outputs match for provided samples")

    final_code = rust_out_path.read_text(encoding="utf-8")
    final_target = Path("rust") / c_path.with_suffix(".rs").name
    final_target.parent.mkdir(parents=True, exist_ok=True)
    final_target.write_text(final_code, encoding="utf-8")
    logger.info("Saved final Rust to %s", final_target)

    write_final_report(c_path, final_target, prompt_dir, args.api_key, work_dir)


if __name__ == "__main__":
    main()


def write_final_report(c_path: Path, rust_path: Path, prompt_dir: Path, api_key: str | None, work_dir: Path) -> None:
    prompt_file = prompt_dir / "final_judge_prompt.txt"
    prompt = load_prompt(prompt_file)
    c_code = c_path.read_text(encoding="utf-8")
    rust_code = rust_path.read_text(encoding="utf-8")
    messages = [
        {"role": "system", "content": "You are a precise C/Rust equivalence reviewer."},
        {
            "role": "user",
            "content": prompt.format(c_name=c_path.name, c_code=c_code, rust_name=rust_path.name, rust_code=rust_code),
        },
    ]
    resp = call_deepseek(messages, api_key=api_key, max_tokens=2048)
    report_path = work_dir / f"report_{c_path.stem}.md"
    work_dir.mkdir(parents=True, exist_ok=True)
    report_path.write_text(resp, encoding="utf-8")
    logger.info("Wrote final equivalence report to %s", report_path)
