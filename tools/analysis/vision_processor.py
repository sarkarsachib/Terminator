from __future__ import annotations


def analyze(image: bytes) -> dict[str, int]:
    return {"bytes": len(image)}
