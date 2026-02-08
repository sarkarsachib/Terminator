from __future__ import annotations
from dataclasses import dataclass, field
from datetime import datetime
from typing import List, Optional, Dict, Any

@dataclass
class PatternObservation:
    pattern_id: str
    modality: str  # vision, audio, motion, internet, text
    confidence: float
    novelty_score: float
    temporal_context: Dict[str, str]  # Simplified from TimestampRange
    spatial_context: Dict[str, Any]
    related_patterns: List[str] = field(default_factory=list)
    causal_predecessors: List[str] = field(default_factory=list)
    timestamp: datetime = field(default_factory=datetime.now)

@dataclass
class TrainedKnowledge:
    concept: str
    domain: str
    source_library: str
    confidence: float
    prerequisites: List[str] = field(default_factory=list)
    related_concepts: List[str] = field(default_factory=list)
    application_domains: List[str] = field(default_factory=list)
    expert_reasoning: str = ""

@dataclass
class UnifiedBelief:
    topic: str
    observer_evidence: float
    trainer_evidence: float
    fusion_confidence: float
    meta_understanding: str
    context_dependent: bool = False
    exception_cases: List[str] = field(default_factory=list)

@dataclass
class LearningMetrics:
    total_patterns_discovered: int = 0
    knowledge_domains_trained: int = 0
    conflict_resolution_rate: float = 0.0
    meta_learning_improvement: float = 0.0
    adaptation_efficiency: float = 0.0
    learning_velocity: float = 0.0
