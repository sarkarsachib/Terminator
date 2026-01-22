from __future__ import annotations

from dataclasses import dataclass


@dataclass(frozen=True)
class FusionInput:
    observer_payload: bytes
    trainer_payload: bytes


def fuse(inp: FusionInput) -> bytes:
    return inp.observer_payload + b"\n" + inp.trainer_payload
