from __future__ import annotations


def mean(xs: list[float]) -> float:
    return sum(xs) / len(xs) if xs else 0.0
