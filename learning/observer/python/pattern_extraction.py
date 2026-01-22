from __future__ import annotations

from dataclasses import dataclass

from .sensors import RawObservation


@dataclass(frozen=True)
class Pattern:
    kind: str
    confidence: float
    payload: bytes


def extract_pattern(obs: RawObservation) -> Pattern:
    return Pattern(kind=f"pattern:{obs.topic}", confidence=0.1, payload=obs.payload)
