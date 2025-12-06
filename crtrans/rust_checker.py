import logging
import subprocess
from pathlib import Path
from typing import Tuple

logger = logging.getLogger(__name__)


def compile_rust(src_path: Path) -> Tuple[bool, str]:
    cmd = ["rustc", "--edition", "2021", str(src_path), "-o", str(src_path.with_suffix(""))]
    logger.info("Compiling Rust: %s", " ".join(cmd))
    proc = subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    ok = proc.returncode == 0
    stderr = proc.stderr.decode()
    if not ok:
        logger.error("rustc failed:\n%s", stderr)
    return ok, stderr
