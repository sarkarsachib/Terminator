from __future__ import annotations


def embed(payload: bytes) -> list[float]:
    return [float(b) / 255.0 for b in payload[:64]]
