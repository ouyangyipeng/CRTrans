from __future__ import annotations

import logging
import re
from dataclasses import dataclass, field
from pathlib import Path
from typing import Dict, List, Set

logger = logging.getLogger(__name__)

try:
    from clang import cindex
    try:
        cindex.Config.set_library_file("/usr/lib/llvm-15/lib/libclang.so.1")
        _CINDEX_AVAILABLE = True
    except Exception:  # noqa: BLE001
        _CINDEX_AVAILABLE = False
except ImportError:
    cindex = None
    _CINDEX_AVAILABLE = False


@dataclass
class Feature:
    name: str
    kind: str  # function, struct, enum, typedef, var
    code: str
    deps: Set[str] = field(default_factory=set)


class CFeatureExtractor:
    def __init__(self, c_path: Path, clang_args: List[str] | None = None):
        self.c_path = c_path
        self.clang_args = clang_args or []
        self._index = None
        if _CINDEX_AVAILABLE:
            try:
                self._index = cindex.Index.create()
            except Exception:  # noqa: BLE001
                logger.warning("libclang present but failed to initialize; using regex fallback")

    def parse(self) -> List[Feature]:
        if self._index is None:
            logger.warning("libclang unavailable; falling back to regex feature extraction")
            return self._fallback_parse()

        tu = self._index.parse(str(self.c_path), args=self.clang_args)
        features: List[Feature] = []
        for child in tu.cursor.get_children():
            if child.location.file and Path(child.location.file.name) != self.c_path:
                continue
            if child.kind == cindex.CursorKind.FUNCTION_DECL and child.is_definition():
                code = self._get_extent(child)
                deps = self._collect_calls(child)
                features.append(Feature(child.spelling, "function", code, deps))
            elif child.kind == cindex.CursorKind.STRUCT_DECL:
                code = self._get_extent(child)
                features.append(Feature(child.spelling or "<anon_struct>", "struct", code, set()))
            elif child.kind == cindex.CursorKind.ENUM_DECL:
                code = self._get_extent(child)
                features.append(Feature(child.spelling or "<anon_enum>", "enum", code, set()))
            elif child.kind == cindex.CursorKind.TYPEDEF_DECL:
                code = self._get_extent(child)
                features.append(Feature(child.spelling, "typedef", code, set()))
            elif child.kind == cindex.CursorKind.VAR_DECL:
                code = self._get_extent(child)
                features.append(Feature(child.spelling, "var", code, set()))
        return features

    def _fallback_parse(self) -> List[Feature]:
        text = self.c_path.read_text(encoding="utf-8")
        pattern = re.compile(r"\b([\w\*\s]+?)\s+(\w+)\s*\(([^)]*)\)\s*{", re.MULTILINE)
        features: List[Feature] = []
        for match in pattern.finditer(text):
            name = match.group(2)
            # crude body capture
            start = match.start()
            brace = text.find("{", start)
            depth = 0
            end = brace
            for i in range(brace, len(text)):
                if text[i] == "{":
                    depth += 1
                elif text[i] == "}":
                    depth -= 1
                    if depth == 0:
                        end = i + 1
                        break
            code = text[start:end]
            deps = set(re.findall(r"(\w+)\s*\(", code)) - {name}
            features.append(Feature(name, "function", code, deps))
        return features

    def _collect_calls(self, node) -> Set[str]:
        calls: Set[str] = set()
        for c in node.get_children():
            if c.kind == cindex.CursorKind.CALL_EXPR:
                calls.add(c.spelling)
            calls |= self._collect_calls(c)
        return calls

    def _get_extent(self, node) -> str:
        start = node.extent.start
        end = node.extent.end
        with open(start.file.name, "r", encoding="utf-8") as f:
            lines = f.readlines()
        return "".join(lines[start.line - 1 : end.line])


def topo_sort(features: List[Feature]) -> List[Feature]:
    graph: Dict[str, Set[str]] = {f.name: set(f.deps) for f in features if f.kind == "function"}
    visited: Set[str] = set()
    order: List[str] = []

    def dfs(n: str):
        if n in visited:
            return
        visited.add(n)
        for dep in graph.get(n, []):
            if dep in graph:
                dfs(dep)
        order.append(n)

    for f in graph:
        dfs(f)

    name_map = {f.name: f for f in features}
    ordered = [name_map[name] for name in order if name in name_map]
    rest = [f for f in features if f not in ordered]
    return ordered + rest
