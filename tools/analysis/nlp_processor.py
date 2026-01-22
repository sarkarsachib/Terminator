from __future__ import annotations


def analyze(text: bytes) -> dict[str, int]:
    return {"bytes": len(text)}
