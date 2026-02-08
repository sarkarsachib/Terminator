from __future__ import annotations
from typing import List, Dict, Any, Optional
from learning.shared.types import PatternObservation

class RealtimeMemory:
    """
    Manages short-term and medium-term storage of discovered patterns.
    """
    def __init__(self):
        self.short_term: List[PatternObservation] = []
        self.medium_term: List[PatternObservation] = []

    def add_observation(self, observation: PatternObservation) -> None:
        self.short_term.append(observation)
        if len(self.short_term) > 1000:
            self.consolidate_to_longterm()

    def query_recent(self, limit: int = 10) -> List[PatternObservation]:
        return self.short_term[-limit:]

    def consolidate_to_longterm(self) -> None:
        # Move patterns from short/medium term to main knowledge graph
        self.medium_term.extend(self.short_term)
        self.short_term = []

    def tag_episodes(self, observation: PatternObservation) -> List[str]:
        # Add episodic context (time, place, situation)
        return ["context:default"]

    def link_to_knowledge_graph(self) -> None:
        # Create edges in the main knowledge graph
        pass

    def retrieve_sensory_context(self, pattern_id: str) -> Dict[str, Any]:
        # Get the raw sensory data associated with a pattern
        return {}
