from __future__ import annotations
from learning.observer.sensors import SensorFusion
from learning.observer.pattern_extraction import PatternExtractor
from learning.trainer.knowledge_loader import KnowledgeLibrary
from learning.hybrid.fusion_engine import FusionEngine
from learning.hybrid.learning_state import LearningState

def main():
    print("--- Terminator Learning System Demo ---")
    
    # 1. Observer in action
    sensor_fusion = SensorFusion()
    extractor = PatternExtractor()
    
    buffer = sensor_fusion.aggregate_streams()
    features = sensor_fusion.extract_features(sensor_fusion.normalize_streams(buffer))
    patterns = extractor.detect_temporal_patterns(features)
    print(f"Observer: Discovered {len(patterns)} patterns from sensor streams.")
    
    # 2. Trainer in action
    library = KnowledgeLibrary()
    library.load_library("philosophy")
    concepts = library.extract_concepts()
    print(f"Trainer: Loaded library with {len(concepts)} concepts.")
    
    # 3. Hybrid Fusion in action
    fusion = FusionEngine()
    state = LearningState()
    
    unified_belief = fusion.fuse_knowledge_sources(patterns, library)
    state.store_knowledge(unified_belief.topic, unified_belief)
    
    print(f"Hybrid: Created unified belief on '{unified_belief.topic}' with confidence {unified_belief.fusion_confidence}")
    print("--- Demo Complete ---")

if __name__ == "__main__":
    main()
