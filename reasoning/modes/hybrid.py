from __future__ import annotations

from .subsymbolic import score


def hybrid_reason(signal: bytes) -> dict[str, float]:
    return {"subsymbolic_score": score(signal)}
