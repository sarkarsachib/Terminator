from __future__ import annotations

from dataclasses import dataclass
from typing import Iterable


@dataclass(frozen=True)
class KnowledgeChunk:
    source: str
    payload: bytes


def load_knowledge(chunks: Iterable[bytes], source: str) -> Iterable[KnowledgeChunk]:
    for chunk in chunks:
        yield KnowledgeChunk(source=source, payload=chunk)
