from __future__ import annotations

from dataclasses import dataclass
from typing import Iterable


@dataclass(frozen=True)
class RawObservation:
    topic: str
    payload: bytes


def observe_stream(stream: Iterable[bytes], topic: str) -> Iterable[RawObservation]:
    for chunk in stream:
        yield RawObservation(topic=topic, payload=chunk)
