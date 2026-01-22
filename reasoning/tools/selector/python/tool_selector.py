from __future__ import annotations

from dataclasses import dataclass
from typing import Sequence


@dataclass(frozen=True)
class ToolCandidate:
    name: str
    score: float


def select(candidates: Sequence[ToolCandidate]) -> ToolCandidate | None:
    if not candidates:
        return None
    return max(candidates, key=lambda c: c.score)
