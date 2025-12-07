# CRTrans：面向单文件的 C→Rust LLM 增强转译流水线（中文详版）

本工程旨在用可复现的 CLI 流水线，将单个 C 源文件自动转译为尽量安全、可编译、行为对齐的 Rust 代码。流水线结合静态分析（libclang、c2rust）、多轮 LLM 提示、rustc 语法/语义纠错循环，以及最终由 LLM 对 C/Rust 结果进行等价性复核并生成报告。

## 总览与创新点

- **模块化分层**：解析（libclang/regex）、静态迁移提示（c2rust）、签名候选生成、函数级翻译、rustc 语法/语义纠错、输出对比修复、最终等价性审查，均拆成独立模块与提示文件，可替换、可单测。
- **多源上下文融合**：C AST 依赖、c2rust 静态 hint、已翻译 callees 签名同步供给 LLM，降低类型/指针歧义。
- **双循环纠错**：
  - 语法/类型循环：rustc 报错→自动/LLM 修复→再编译。
  - 语义/IO 循环：`info.md` 样例输入比对 C/Rust 输出→LLM 修复→再比对。
- **确定性兜底**：libclang 可用时优先；不可用自动切换 regex，无告警噪声。LLM info 解析失败时自动构造描述与样例，保持流水线不中断。
- **最终审查报告**：收敛后将最终 C/Rust 代码提交给 LLM 做一次“等价性复核”，输出 Markdown 报告供人工审阅。

## 流程图（Mermaid）

```mermaid
flowchart TD
  A[选择单个 C 文件] --> B[生成 compile_commands.json]
  A --> C[info.md: LLM 描述与样例, 运行 C 得输出]
  A --> D[解析 C: libclang / regex]
  B --> E[c2rust 生成静态 hint]
  D --> F[按依赖拓扑遍历特征]
  F --> G[签名候选生成]
  G --> H[函数翻译 (结合 callees + hint)]
  H --> I[组装 Rust 文件]
  I --> J[rustc 编译循环]
  J --> K[运行 Rust 样例输出]
  K --> L[与 C 输出比对]
  L -->|不一致| H
  L -->|一致| M[保存最终 Rust]
  M --> N[最终 LLM 等价性审查报告]
```

## 目录与模块说明

- `transpile.py`：CLI 入口，核心参数 `--c-file`、`--api-key`、`--max-fix-iters`（默认 10）；自动按源文件名建立 `temp/<safe_stem>/` 工作区，组装/修复均在该目录；仅最终成果落盘 `rust/<safe_stem>.rs`。
- `crtrans/`
  - `logging_setup.py`：控制台 + 旋转文件日志，统一格式。
  - `c_parser.py`：优先加载 `/usr/lib/llvm-14/lib/libclang.so(.1)`（可用 `LIBCLANG_PATH` 覆盖）；成功则用 libclang 抽取函数/struct/enum/typedef/var 及调用依赖；失败自动 regex，零告警降级。
  - `c2rust_wrapper.py`：写 `compile_commands.json`，清理输出目录后调用 c2rust 生成静态 Rust hint（供类型参考）。
  - `info_builder.py`：调用 LLM 生成描述/样例并剥除 code fence，解析失败则确定性兜底；编译运行 C 收集样例输出，写入 `temp/<safe_stem>/info.md`；编译/运行失败会立即报错并终止。
  - `prompting.py`：加载提示模板并调用 DeepSeek API。
  - `translator.py`：签名候选解析（JSON/文本多级兜底）、函数翻译、组装去重；提示词收紧为“只返回代码”。
  - `rust_checker.py`：rustc 编译封装。
  - `runner.py`：运行二进制并比对 stdout/stderr/rc。
- `prompt/`：分模块提示，包括 info、签名、翻译、rustc 修复、输出修复、最终审查等。
- `temp/`：每个输入对应 `temp/<safe_stem>/`，包含中间产物（翻译稿、二进制、info、c2rust、日志、最终审查报告）。
- `rust/`：仅保存最终收敛的 Rust 副本。

## 详细流水线步骤

1. **输入定位与准备**
  - CLI 解析 `--c-file`；缺省时要求 `C/` 仅有一个 `.c`。
  - 构建 `compile_commands.json`（O0），为 c2rust 与 libclang 提供统一编译上下文；文件名与输出名加引号并安全化，兼容空格。
2. **信息与样例生成（`info_builder`）**
  - 向 LLM 发送源码与 `info_prompt`，期望 JSON（description/samples/notes），剥除 Markdown fence 后解析。
  - 解析失败的确定性兜底：填充固定描述与最小样例，保证后续可运行。
  - 编译并运行 C，逐一喂入样例，记录 stdout/stderr/rc，写入 `temp/<safe_stem>/info.md` 作为对照输出；运行失败即停止。
3. **静态迁移提示（`c2rust_wrapper`）**
  - 清理输出目录，调用 c2rust 生成静态 Rust hint（可能含 unsafe），用作类型/签名参考但不直接信任。
4. **特征抽取与依赖排序（`c_parser`）**
  - 优先加载 `/usr/lib/llvm-14/lib/libclang.so(.1)`（可用 `LIBCLANG_PATH` 覆盖）；成功则遍历 AST，提取函数/struct/enum/typedef/var 及调用依赖。
  - libclang 不可用自动 regex，零告警降级。
  - 构建调用依赖图，做简易拓扑排序，保证翻译时已有 callee 签名可用。
5. **签名候选生成（`translator.generate_signatures`）**
  - 结合 c2rust hint 和推荐指针映射，要求 LLM 返回 1~2 个惯用 Rust 签名（仅返回纯文本）。
  - 解析链：去 fence → 尝试 JSON → 从文本提取 `fn ...` → 兜底空签名，避免“parse failed”终止。
6. **函数体翻译（`translator.translate_function`）**
  - 上下文包含目标签名、callee 签名列表、静态 hint、原 C 代码。
  - 提示强调少 unsafe、偏向引用/切片/Result/Option；提示强制“只返回 Rust 代码”，自动剥除 fence/语言标签。
  - 逐函数累积翻译结果，重复函数名会在组装阶段去重。
7. **组装与去重（`translator.assemble_rust`）**
  - 按拓扑序拼接函数与定义，缺失则填编译可过的 stub。
  - `_dedup_functions`：重复 `fn name` 仅保留首个版本，避免重复定义导致 rustc 失败。
8. **语法/类型修复循环（`rust_checker` + `fix_prompt`）**
  - 编译 `temp/<safe_stem>/translated.rs`；若 rustc 报错，将错误与源码送入 LLM 生成修订版并重编译。
  - 循环上限 `--max-fix-iters=10`（可按文件规模自适应上调），确保编译通过后再进入语义比对。
9. **语义/IO 比对循环（`runner` + `compare_fix_prompt`）**
  - 使用 `info.md` 样例运行 C/Rust，比较 stdout/rc。
  - 不一致则携差异提示 LLM 修复，再编译再比对；语义循环与语法循环共用统一预算，编译失败优先；差异详情仅写入日志文件（debug），控制台保持简洁。
10. **产出与归档**
  - 最终可编译版写入 `rust/<safe_stem>.rs`（中间稿留在 `temp/<safe_stem>/`）。
  - 触发最终等价性审查：将 C/Rust 全量代码与 `final_judge_prompt` 交给 LLM，生成 Markdown 报告 `temp/<safe_stem>/report_<safe_stem>.md`，仅供审阅，不改代码。

## 关键技术要点（展开）

- **libclang 优先，regex 兜底**：默认加载 14 版 libclang，路径可配置；失配自动静默切换 regex，避免告警噪声。
- **签名解析防御链**：去 fence → JSON → 文本提取 → 兜底空签名，多级确保无“解析失败”中断。
- **双循环、统一上限**：语法/语义修复共用 10 次迭代预算，且编译优先，避免语义修复引入新语法错误导致死锁。
- **静态 hint 融合**：c2rust 产物与已翻译 callee 签名同时喂给 LLM，降低指针/类型漂移风险。
- **确定性样例兜底**：LLM info 解析失败也能落地最小样例，保证流水线端到端可运行。
- **最终等价性审查**：LLM 对 C/Rust 全量代码做独立审阅，输出风险/差异 Markdown 供人工决策。

## 使用方法

```bash
# 运行一个 C 文件（例：bubble_sort）
/usr/bin/python transpile.py --c-file C/bubble_sort.c --api-key $DEEPSEEK_KEY

# 重要可选环境变量
# LIBCLANG_PATH  指定自定义 libclang.so 路径
# ENABLE_LIBCLANG=1 强制使用 libclang（默认自动尝试 14 版路径）
```

输出位置：
- `temp/<safe_stem>/translated.rs`：当前迭代的 assembled Rust。
- `rust/<safe_stem>.rs`：收敛后的最终副本。
- `temp/<safe_stem>/info.md`：描述 + 样例输入输出。
- `temp/<safe_stem>/c2rust/`：c2rust 静态提示。
- `temp/<safe_stem>/report_<safe_stem>.md`：最终 LLM 等价性审查报告。
- `temp/<safe_stem>/logs/crtrans.log`：运行日志，含详细 diff（debug）。

## 依赖与环境

- Python 3.10+，依赖：`requests`, `clang`。
- libclang：已内置路径 `/usr/lib/llvm-14/lib/libclang.so(.1)`，可用 `LIBCLANG_PATH` 覆盖。
- Rust 工具链（rustc 2021 edition），c2rust 已安装。
- DeepSeek API key 通过参数或环境变量提供。

## 局限与后续改进方向

- 复杂跨文件依赖未覆盖；当前假设单文件输入。
- 样例生成仍依赖 LLM / 简单兜底，未集成符号执行工具。
- 类型迁移策略可进一步结合运行时反馈做自动签名搜索。
- 目前修复循环次数固定，可基于编译/比对改进自适应退出策略。

## 快速回顾

CRTrans 将静态工具与 LLM 协同：用静态 hint 降低类型不确定性，用 rustc/输出比对闭环纠错，并提供最终审查报告，为 C→Rust 单文件迁移提供可落地、可审计的自动化基线。
