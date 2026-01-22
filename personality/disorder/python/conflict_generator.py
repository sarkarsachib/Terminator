from __future__ import annotations

from dataclasses import dataclass


@dataclass(frozen=True)
class Conflict:
    a: str
    b: str


def generate(a: str, b: str) -> Conflict:
    return Conflict(a=a, b=b)
