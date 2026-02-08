import unittest
from learning.shared.types import PatternObservation, TrainedKnowledge, UnifiedBelief
from learning.observer.sensors import SensorFusion
from learning.trainer.knowledge_loader import KnowledgeLibrary
from learning.hybrid.conflict_resolver import ConflictResolver

class TestLearningSystem(unittest.TestCase):
    def test_sensor_fusion_normalization(self):
        sf = SensorFusion()
        buffer = sf.aggregate_streams()
        normalized = sf.normalize_streams(buffer)
        self.assertTrue(normalized["normalized"])
        self.assertIn("vision", normalized["sources"])

    def test_knowledge_loader(self):
        kl = KnowledgeLibrary()
        lib = kl.load_library("test_domain")
        self.assertEqual(lib["domain"], "test_domain")

    def test_conflict_resolution(self):
        cr = ConflictResolver()
        belief = cr.synthesize_knowledge(None)
        self.assertEqual(belief.topic, "resolved")
        self.assertGreater(belief.fusion_confidence, 0.0)

if __name__ == '__main__':
    unittest.main()
