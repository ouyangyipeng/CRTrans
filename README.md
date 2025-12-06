# CRTrans：面向单文件的 C→Rust LLM 增强转译流水线（中文详版）

本工程旨在用可复现的 CLI 流水线，将单个 C 源文件自动转译为尽量安全、可编译、行为对齐的 Rust 代码。流水线结合静态分析（libclang、c2rust）、多轮 LLM 提示、rustc 语法/语义纠错循环，以及最终由 LLM 对 C/Rust 结果进行等价性复核并生成报告。

## 总览与创新点

- **模块化分层**：解析（libclang/regex）、静态迁移提示（c2rust）、签名候选生成、函数级翻译、rustc/输出对比修复、最终等价性审查，均拆成独立模块与提示文件。
- **多源上下文融合**：C AST 依赖、c2rust 生成的静态 hint、已翻译的 callees 签名共同喂给 LLM，降低类型/指针歧义。
- **双循环纠错**：
  - 语法/类型循环：rustc 报错→自动/LLM 修复→再编译。
  - 语义/IO 循环：基于 `info.md` 样例输入比对 C/Rust 输出→LLM 修复→再比对。
- **确定性兜底**：
  - libclang 可用时优先；不可用自动切换 regex，无告警噪声。
  - LLM info 解析失败时，自动构造描述与样例，保证流水线不中断。
- **最终审查报告**：收敛后将最终 C/Rust 代码提交给 LLM 做一次“等价性复核”并输出 Markdown 报告，便于人工审阅。

## 流程图（Mermaid）

```mermaid
flowchart TD
   A[选择单个 C 文件] --> B[生成 compile_commands.json]
   B --> C[c2rust 生成静态 hint]
   A --> D[info.md: LLM 描述+样例; 运行 C 获取参考输出]
   A --> E[解析 C: libclang or regex 提取函数/依赖]
   E --> F[按拓扑序遍历特征]
   F --> G[签名候选生成 (LLM)]
   G --> H[函数翻译 (LLM, 结合 callees + static hint)]
   H --> I[组装 Rust 文件]
   I --> J[rustc 编译循环 (LLM 修复)]
   J --> K[运行 Rust 样例输出]
   K --> L[与 C 输出比对]
   L -->|不一致| H
   L -->|一致| M[保存最终 Rust]
   M --> N[最终 LLM 等价性审查，生成报告]
```

## 目录与模块说明

- `transpile.py`：主 CLI，串联各阶段并负责两层修复循环与最终报告。
- `crtrans/`
  - `logging_setup.py`：控制台 + 旋转文件日志。
  - `c_parser.py`：优先加载 `/usr/lib/llvm-14/lib/libclang.so(.1)`（或 `LIBCLANG_PATH`），成功则用 libclang 抽取函数/结构/依赖；否则自动用 regex 解析，零告警降级。
  - `c2rust_wrapper.py`：写 `compile_commands.json`，清理输出目录后调用 c2rust 生成静态 Rust hint。
  - `info_builder.py`：调用 LLM 生成描述/样例，若解析失败则确定性兜底；编译运行 C 收集样例输出，写入 `temp/info.md`。
  - `prompting.py`：加载提示模板并调用 DeepSeek API。
  - `translator.py`：签名候选解析（鲁棒 JSON/文本）、函数翻译、去重组装。
  - `rust_checker.py`：rustc 编译封装。
  - `runner.py`：运行二进制并比对输出。
- `prompt/`：分模块提示，包括 info、签名、翻译、rustc 修复、输出修复、最终审查等。
- `temp/`：中间产物与日志、最终审查报告 `report_<cfile>.md`。
- `rust/`：最终 Rust 输出副本。

## 详细流水线步骤

1. **输入确定**：
  - 若 CLI 指定 `--c-file` 则直接使用；否则要求 `C/` 中唯一的 `.c`。
2. **信息与样例生成 (`info_builder`)**：
  - 调用 LLM 产出功能描述/样例；若返回无法解析，自动兜底样例。
  - 编译并运行 C，记录样例输出，生成 `temp/info.md`。
3. **静态 hint (`c2rust_wrapper`)**：
  - 生成 `compile_commands.json`。
  - 清理输出目录后调用 c2rust，供后续类型参考。
4. **特征抽取 (`c_parser`)**：
  - 优先 libclang 抽取函数/依赖；失败自动 regex，无警告噪声。
  - 按调用依赖做简易拓扑排序。
5. **函数级翻译 (`translator`)**：
  - 对每个函数：
    - LLM 生成 1~2 个 Rust 签名候选（结合静态 hint / 推荐映射）。
    - 选择首个候选（或兜底 stub）。
    - 提供 callees 签名与 c2rust hint，LLM 翻译函数体。
  - 汇总并去重后组装 Rust 源。
6. **rustc 语法/类型修复循环**：
  - 编译失败时，将源码与 rustc 报错送入 LLM 生成修复版本，迭代上限 `--max-fix-iters`。
7. **语义/IO 比对循环**：
  - 运行 Rust 与 C 的样例，比较 stdout/rc。
  - 不一致则将差异送入 LLM 生成修复，再编译再比对，迭代上限 `--max-fix-iters`。
8. **产出**：
  - 最终 Rust 写入 `rust/<cfile>.rs` 与 `rust/translated.rs`（工作文件）。
  - 生成最终等价性审查报告 `temp/report_<cfile>.md`（LLM 对 C/Rust 全量审阅）。

## 关键技术要点

- **libclang + regex 双通道**：优先使用给定的 14 版 libclang；若符号不兼容自动回落 regex，保证无警告且不中断。
- **签名鲁棒解析**：去除 Markdown fence 后尝试 JSON 解析；若失败从文本抽取 `fn ...`，最后兜底生成空签名，避免“解析失败”告警。
- **双重修复循环**：rustc 报错驱动的语法/类型修复，与样例输出驱动的语义修复分别迭代，减小单次 LLM 失误带来的卡死。
- **确定性样例兜底**：LLM info 解析失败时，也能自动生成最小样例，保障流水线端到端可运行。
- **最终 LLM 审查报告**：不修改代码，只输出 Markdown，便于人工快速审阅风险与差异。

## 使用方法

```bash
# 运行一个 C 文件（例：bubble_sort）
/usr/bin/python transpile.py --c-file C/bubble_sort.c --api-key $DEEPSEEK_KEY

# 重要可选环境变量
# LIBCLANG_PATH  指定自定义 libclang.so 路径
# ENABLE_LIBCLANG=1 强制使用 libclang（默认自动尝试 14 版路径）
```

输出位置：
- `rust/translated.rs`：当前迭代的 assembled Rust。
- `rust/<cfile>.rs`：收敛后的最终副本。
- `temp/info.md`：描述 + 样例输入输出。
- `temp/c2rust/`：c2rust 静态提示。
- `temp/report_<cfile>.md`：最终 LLM 等价性审查报告。
- `temp/logs/crtrans.log`：运行日志。

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
