from __future__ import annotations

import random
from dataclasses import dataclass


@dataclass(frozen=True)
class DisorderParams:
    intensity: float
    trigger_rate: float


def should_deviate(params: DisorderParams, seed: int | None = None) -> bool:
    rng = random.Random(seed)
    return rng.random() < max(0.0, min(1.0, params.trigger_rate * (1.0 + params.intensity)))
