from __future__ import annotations

from dataclasses import dataclass


@dataclass(frozen=True)
class Frame:
    payload: bytes


def process(frame: Frame) -> bytes:
    return frame.payload
