from __future__ import annotations

from dataclasses import dataclass


@dataclass(frozen=True)
class Conflict:
    left: bytes
    right: bytes


def resolve(conflict: Conflict) -> bytes:
    return conflict.left if len(conflict.left) >= len(conflict.right) else conflict.right
