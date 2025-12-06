import json
import logging
import subprocess
from pathlib import Path
from typing import List

logger = logging.getLogger(__name__)


def write_compile_commands(c_file: Path, compile_commands: Path) -> None:
    compile_commands.parent.mkdir(parents=True, exist_ok=True)
    entry = {
        "directory": str(c_file.parent.resolve()),
        "command": f"clang -O0 -c {c_file.name} -o {c_file.stem}.o",
        "file": c_file.name,
    }
    compile_commands.write_text(json.dumps([entry], indent=4), encoding="utf-8")
    logger.info("Wrote compile_commands.json at %s", compile_commands)


def run_c2rust(compile_commands: Path, output_dir: Path) -> Path:
    output_dir.mkdir(parents=True, exist_ok=True)
    cmd: List[str] = [
        "c2rust",
        "transpile",
        "--emit-build-files",
        "--output-dir",
        str(output_dir),
        str(compile_commands),
    ]
    logger.info("Running c2rust: %s", " ".join(cmd))
    subprocess.run(cmd, check=True)
    candidate = output_dir / "src" / (compile_commands.parent.name + "_c.rs")
    # Fallback if name unknown; caller can adjust.
    return candidate
