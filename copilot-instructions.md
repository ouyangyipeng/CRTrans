# Copilot Instructions for CRTrans

Purpose: single-file C→Rust LLM-assisted pipeline with deterministic scaffolding, static hints (libclang + c2rust), iterative rustc/output repair, and final LLM equivalence review. Default language: Chinese docs; code is English.

## Key Defaults
- Model: `deepseek-reasoner` at `https://api.deepseek.com` (temperature 0.0). Thinking mode toggled by CLI flag `--turbo` (adds `thinking: {"type": "enabled"}` to requests).
- CLI: `python transpile.py --c-file <path> --api-key <key> [--turbo] [--work-dir <dir>] [--rust-out <path>] [--max-fix-iters N]`.
- Work dirs: per run at `temp/<safe_stem>/`; final Rust saved to `rust/<safe_stem>.rs`. Intermediate translated stays in the per-file temp dir by default.
- Safe names: stems sanitized with `[^A-Za-z0-9_]+ -> _` to handle spaces.

## Pipeline (condensed)
1) Build `compile_commands.json` (O0, quoted paths) for c2rust/libclang.
2) `info_builder`: LLM description + samples (code-only responses); fallback deterministic sample; compile/run C to log outputs in `temp/<safe_stem>/info.md`.
3) `c2rust_wrapper`: static Rust hints in `temp/<safe_stem>/c2rust/` (advisory only).
4) `c_parser`: libclang first (override `LIBCLANG_PATH`), regex fallback, topo order on deps.
5) `translator.generate_signatures`: asks for 1–2 Rust signatures, code-only.
6) `translator.translate_function`: code-only Rust bodies, minimal unsafe, uses callees + static hints.
7) Assemble + dedup functions; stub missing pieces.
8) Fix loop (syntax/type): rustc → if fail, LLM fix with `fix_prompt`; budget derived from file size (min CLI max, upper 50).
9) Output loop: run Rust vs C samples; mismatches to LLM via `compare_fix_prompt`; detailed diffs logged at debug only.
10) Final: copy to `rust/<safe_stem>.rs`; run final equivalence review LLM, write `temp/<safe_stem>/report_<safe_stem>.md`.

## Logging & Noise
- Console: high level only; diffs and rustc errors logged at debug in `temp/<safe_stem>/logs/crtrans.log`.
- Mismatch spam suppressed on stdout; review log for details.

## Dependencies & Setup
- Python 3.10+; `pip install -r requirements.txt` (requests, urllib3, clang==14.0.6).
- System: clang/llvm/libclang 14 (override with `LIBCLANG_PATH`), rustc 2021 toolchain, `c2rust` in PATH.
- Recommended setup (Ubuntu 22.04 + conda) is documented in README; venv/mac alternatives also noted.

## Prompts & Response Hygiene
- Prompts require code-only outputs: no markdown fences, no explanations.
- Thinking mode only when `--turbo`; otherwise standard responses.

## What to maintain going forward
- Keep CLI/README in sync when flags/defaults change (model, turbo behavior, paths).
- Preserve per-file isolation in `temp/<safe_stem>/`; never emit intermediates to repo root.
- Continue quoting paths in compile_commands and shell invocations to support spaces.
- When adding tools or prompts, ensure outputs respect code-only contract and log noise stays low.
- Honor iteration budget logic; avoid regressing to fixed small loops.

## How to run a test example
```
python transpile.py --c-file C/github/Age_in_Days_Months_Year.c --api-key "$DEEPSEEK_KEY" --turbo
```
Outputs to `temp/Age_in_Days_Months_Year/` and final Rust to `rust/Age_in_Days_Months_Year.rs`.
