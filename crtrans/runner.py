from __future__ import annotations

import logging
import subprocess
from pathlib import Path
from typing import List, Dict

logger = logging.getLogger(__name__)


def run_binary(bin_path: Path, inputs: List[str]) -> List[Dict[str, str]]:
    results = []
    for inp in inputs:
        proc = subprocess.run(
            [str(bin_path)],
            input=inp.encode(),
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
        )
        results.append(
            {
                "input": inp,
                "stdout": proc.stdout.decode(),
                "stderr": proc.stderr.decode(),
                "rc": str(proc.returncode),
            }
        )
    return results


def compare_outputs(c_outputs: List[Dict[str, str]], r_outputs: List[Dict[str, str]]) -> List[str]:
    diffs: List[str] = []
    for i, (c_out, r_out) in enumerate(zip(c_outputs, r_outputs)):
        c_rc = str(c_out.get("rc", c_out.get("returncode", "")))
        r_rc = str(r_out.get("rc", r_out.get("returncode", "")))
        if c_out.get("stdout") != r_out.get("stdout") or c_rc != r_rc:
            diffs.append(
                f"Sample {i+1} mismatch: C stdout={c_out.get('stdout')!r}, Rust stdout={r_out.get('stdout')!r}, rc C={c_rc}, rc Rust={r_rc}"
            )
    return diffs
