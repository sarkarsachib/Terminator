from __future__ import annotations

from dataclasses import dataclass

from .knowledge_loader import KnowledgeChunk


@dataclass(frozen=True)
class InjectionResult:
    accepted: bool
    reason: str


def inject(chunk: KnowledgeChunk) -> InjectionResult:
    if not chunk.payload:
        return InjectionResult(accepted=False, reason="empty payload")
    return InjectionResult(accepted=True, reason="accepted")
