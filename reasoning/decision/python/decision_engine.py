from __future__ import annotations

from dataclasses import dataclass


@dataclass(frozen=True)
class Decision:
    action: str
    confidence: float


def decide(context: bytes) -> Decision:
    if context:
        return Decision(action="noop", confidence=0.5)
    return Decision(action="idle", confidence=0.1)
