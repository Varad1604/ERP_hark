"""Run with python -m unittest discover -s tools/tests -p 'test_*.py'."""
import importlib.util
import unittest
from pathlib import Path

module_path = Path(__file__).resolve().parents[1] / "a06_j05_registry_evidence_check.py"
spec = importlib.util.spec_from_file_location("checker", module_path)
checker = importlib.util.module_from_spec(spec)
spec.loader.exec_module(checker)

def valid():
    return {
        "provenance": {k: "source-backed-reference" for k in (
            "registryId", "environment", "capturedAt", "operator", "authorityReference", "exportDigest")},
        "definitions": [
            {"name": "ReceiptVarianceRecorded", "status": "ACTIVE",
             **{k: "source-backed-reference" for k in (
                 "definitionId", "version", "schemaId", "schemaSha256",
                 "schemaReference", "activatedAt", "approvalReference", "businessMeaning")},
             "owner": "J05", "defaultVersion": True},
            {"name": "GoodsReceiptVarianceRecorded", "status": "NOT_FOUND"}
        ],
        "consumerChecks": [
            {"owner": owner, "status": "SUPPORTED", "supportedVersion": "v1",
             "registryEvidenceReference": "x", "ownerDecisionReference": "x",
             "compatibilityEvidenceReference": "x"} for owner in ("J06", "J08", "F14")
        ],
        "approval": {k: "independent-proof-required" for k in (
            "a06DecisionReference", "j05DecisionReference", "securityReviewReference")}
    }

class TestEvidenceStructure(unittest.TestCase):
    def test_claimed_complete_structure_only(self):
        self.assertEqual(checker.inspect(valid()), [])
    def test_empty_fail_closed(self):
        self.assertTrue(checker.inspect({}))
    def test_both_active_is_blocked(self):
        sample=valid()
        sample["definitions"][1] = {**sample["definitions"][0],
                                      "name": "GoodsReceiptVarianceRecorded"}
        self.assertIn("requires exactly one Active/default candidate; otherwise reconcile names",
                      checker.inspect(sample))
    def test_missing_consumer_blocks(self):
        sample=valid()
        sample["consumerChecks"].pop()
        self.assertTrue(any("J06, J08 and F14" in x for x in checker.inspect(sample)))
    def test_mismatched_owner_blocks(self):
        sample=valid()
        sample["definitions"][0]["owner"]="A06"
        self.assertTrue(any("does not declare J05 owner" in x for x in checker.inspect(sample)))
    def test_malformed_name_fails_closed_without_exception(self):
        sample=valid()
        sample["definitions"][0]["name"] = []
        self.assertTrue(checker.inspect(sample))
    def test_malformed_status_fails_closed_without_exception(self):
        sample=valid()
        sample["definitions"][0]["status"] = []
        self.assertTrue(checker.inspect(sample))
    def test_malformed_consumer_owner_fails_closed_without_exception(self):
        sample=valid()
        sample["consumerChecks"][0]["owner"] = []
        self.assertTrue(checker.inspect(sample))
    def test_blocked_consumer_rejects_structure(self):
        sample=valid()
        sample["consumerChecks"][0]["status"] = "BLOCKED"
        self.assertTrue(any("unresolved consumer" in x for x in checker.inspect(sample)))
    def test_missing_schema_blocks(self):
        sample=valid()
        del sample["definitions"][0]["schemaSha256"]
        self.assertTrue(any("schemaSha256" in x for x in checker.inspect(sample)))

if __name__ == "__main__":
    unittest.main()
