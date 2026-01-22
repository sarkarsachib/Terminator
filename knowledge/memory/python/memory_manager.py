from __future__ import annotations

from dataclasses import dataclass


@dataclass
class EpisodicMemory:
    events: list[bytes]


class MemoryManager:
    def __init__(self) -> None:
        self.episodic = EpisodicMemory(events=[])

    def remember(self, event: bytes) -> None:
        self.episodic.events.append(event)
