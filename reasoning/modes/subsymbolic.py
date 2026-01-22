from __future__ import annotations


def score(signal: bytes) -> float:
    if not signal:
        return 0.0
    return min(1.0, len(signal) / 1024.0)
