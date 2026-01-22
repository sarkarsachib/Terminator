from __future__ import annotations

from dataclasses import dataclass


@dataclass
class TraitState:
    curiosity: float = 0.5
    caution: float = 0.5
    assertiveness: float = 0.5


def clamp01(x: float) -> float:
    return max(0.0, min(1.0, x))


def apply_feedback(state: TraitState, curiosity_delta: float = 0.0) -> TraitState:
    state.curiosity = clamp01(state.curiosity + curiosity_delta)
    return state
