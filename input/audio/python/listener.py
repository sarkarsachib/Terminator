from __future__ import annotations

from dataclasses import dataclass


@dataclass(frozen=True)
class AudioChunk:
    payload: bytes


def listen() -> AudioChunk:
    return AudioChunk(payload=b"")
