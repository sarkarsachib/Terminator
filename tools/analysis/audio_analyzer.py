from __future__ import annotations


def analyze(audio: bytes) -> dict[str, int]:
    return {"bytes": len(audio)}
