# CRTrans pipeline (plan)

Goals: translate a single C file to Rust with LLM assistance, minimal unsafe, maintain behavior, iterate on rustc errors, and compare outputs on sample inputs.

Structure draft:
- `transpile.py`: main CLI orchestrator. Args: `--c-file` (path in C/), `--out-dir rust/translated`, `--work-dir temp/`, `--api-key` (env or arg), `--max-iters`, `--log-level`.
- `crtrans/` package (plain `.py` files):
  - `logging_setup.py`: configure rotating file + console logging.
  - `c_parser.py`: libclang-based feature extraction (struct/enum/typedef/global/function) and dependency graph (call graph + type refs). Outputs JSON and topo order.
  - `prompting.py`: load prompt templates under `prompt/`, fill variables, call DeepSeek API (chat completion) with retry/backoff.
  - `c2rust_wrapper.py`: write `compile_commands.json`, call c2rust to `temp/static-unsafe-code.rs` (or rust/src if desired).
  - `info_builder.py`: run C program on sample inputs; if not provided, auto-generate 1-3 trivial inputs by static analysis placeholder; ask LLM for problem statement + sample inputs; assemble `info.md` in `temp/`.
  - `translator.py`: per-feature translation using candidate signatures, merges results, saves interim Rust snippets per feature.
  - `rust_checker.py`: compile candidate snippets with `rustc`; apply simple suggested fixes automatically; fallback to LLM patch loop.
  - `runner.py`: run built C and Rust binaries on sample inputs and compare outputs.

Pipeline:
1) Discover C file (single `.c` in provided path). Build compile_commands.json.
2) Generate `info.md`: LLM description + sample inputs; compile/run C to capture outputs.
3) Run c2rust to get `temp/static-unsafe-code.rs` for type hints.
4) Parse C with libclang -> feature list + dependency graph + topo order.
5) For each feature in order: ask LLM for 2 Rust signatures; translate body using selected signature + callee signatures + static unsafe reference; store snippet.
6) Assemble Rust file; rustc compile; loop applying compiler suggestions or LLM fixes until stable or max iterations.
7) Run Rust binary on sample inputs; compare with C outputs. If mismatch, send diff to LLM for patch; repeat within max iterations.
8) Save final Rust to `rust/<cfile>.rs`; logs to `temp/logs/`. Intermediate artifacts live in `temp/`.

Test plan:
- Use existing `C/hello.c` and `C/test.c` as smoke tests.
- CLI: `python transpile.py --c-file C/hello.c --api-key $DEEPSEEK_KEY`.

## Quickstart

1) Install Python deps: `pip install requests clang` (already added). If libclang shared lib is missing, the parser falls back to a regex extractor.
2) Run: `/usr/bin/python transpile.py --c-file C/hello.c --api-key $DEEPSEEK_KEY`
3) Outputs:
  - temp/info.md: LLM description + sample IO
  - temp/c2rust: static unsafe hints from c2rust (best effort)
  - rust/translated.rs: assembled Rust from LLM
  - rust/<name>.rs: final copy
  - temp/logs/crtrans.log: run log

Notes:
- If c2rust is missing or fails, the flow continues with an empty static hint.
- If DeepSeek replies with fenced code, fences are stripped before compiling.

Notes:
- LLM model: DeepSeek, key supplied by user.
- Keep unsafe minimal; prefer references/Box over raw pointers.
- If optional tools (symcc/KLEE) are unavailable, skip and log warning.
