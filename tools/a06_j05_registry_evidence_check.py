#!/usr/bin/env python3
"""Offline, fail-closed examination of an authoritative A06 registry evidence bundle.

Does not access a registry, verify signatures, approve event definitions or authorize
publication. Requires independent provenance validation by A06 and source owners.
"""
import argparse
import json
import sys
from pathlib import Path

NAMES = {"ReceiptVarianceRecorded", "GoodsReceiptVarianceRecorded"}
CONSUMERS = {"J06", "J08", "F14"}

def nonblank(obj, field):
    return isinstance(obj, dict) and isinstance(obj.get(field), str) and bool(obj[field].strip())

def inspect(bundle):
    problems = []
    def need(ok, reason):
        if not ok:
            problems.append(reason)

    need(isinstance(bundle, dict), "bundle must be an object")
    if not isinstance(bundle, dict):
        return problems

    provenance = bundle.get("provenance")
    need(all(nonblank(provenance, x) for x in
             ("registryId", "environment", "capturedAt", "operator", "authorityReference", "exportDigest")),
         "authenticated registry provenance fields absent; digest still needs independent validation")
    defs = bundle.get("definitions")
    need(isinstance(defs, list) and len(defs) >= 1, "both-name registry lookup results required")
    if not isinstance(defs, list):
        return problems
    listed = {x.get("name") for x in defs if isinstance(x, dict) and isinstance(x.get("name"), str)}
    need(listed == NAMES and len(defs) == 2, "include exactly both candidate-name lookup results")
    active = []
    for item in defs:
        if not isinstance(item, dict):
            problems.append("definition lookup result must be object")
            continue
        name = item.get("name", "")
        status = item.get("status")
        need(isinstance(name, str) and name in NAMES, "unrecognized event name")
        need(isinstance(status, str) and status in {"ACTIVE", "NOT_FOUND", "INACTIVE"}, f"lookup status invalid for {name}")
        if status == "ACTIVE":
            active.append(item)
            for field in ("definitionId", "owner", "version", "schemaId", "schemaSha256",
                          "schemaReference", "activatedAt", "approvalReference", "businessMeaning"):
                need(nonblank(item, field), f"active {name} missing {field}")
            need(item.get("owner") == "J05", f"active {name} does not declare J05 owner")
            need(item.get("defaultVersion") is True, f"active {name} is not proven default")
        elif status == "INACTIVE":
            need(nonblank(item, "definitionId"), f"inactive {name} lacks definition identity")
    need(len(active) == 1, "requires exactly one Active/default candidate; otherwise reconcile names")
    subscriptions = bundle.get("consumerChecks")
    need(isinstance(subscriptions, list), "consumer check entries required")
    if isinstance(subscriptions, list):
        names = [c.get("owner") for c in subscriptions if isinstance(c, dict) and isinstance(c.get("owner"), str)]
        need(len(names) == len(set(names)), "duplicate consumer owner entries")
        need(CONSUMERS.issubset(set(names)), "require explicit J06, J08 and F14 consumer checks")
        for c in subscriptions:
            if not isinstance(c, dict):
                problems.append("consumer check must be object")
                continue
            owner = c.get("owner", "<unknown>")
            need(isinstance(c.get("status"), str) and c.get("status") in {"SUPPORTED", "NOT_SUBSCRIBED", "BLOCKED"},
                 f"invalid consumer check status: {owner}")
            need(nonblank(c, "registryEvidenceReference") and nonblank(c, "ownerDecisionReference"),
                 f"missing independent subscriber and owner evidence for {owner}")
            if c.get("status") == "BLOCKED":
                problems.append(f"unresolved consumer blocks structural completeness: {owner}")
            if c.get("status") == "SUPPORTED":
                need(nonblank(c, "supportedVersion") and nonblank(c, "compatibilityEvidenceReference"),
                     f"missing version/compatibility proof for {owner}")
    approval = bundle.get("approval")
    for field in ("a06DecisionReference", "j05DecisionReference", "securityReviewReference"):
        need(nonblank(approval, field), f"missing independent approval evidence: {field}")
    return problems

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("evidence_bundle", type=Path)
    args = parser.parse_args()
    try:
        bundle = json.loads(args.evidence_bundle.read_text(encoding="utf-8"))
    except (OSError, UnicodeError, json.JSONDecodeError) as e:
        print(f"BLOCKED: cannot read evidence bundle: {e}", file=sys.stderr)
        return 2
    problems = inspect(bundle)
    if problems:
        for message in problems:
            print(f"BLOCKED: {message}")
        return 1
    print("STRUCTURE PASS ONLY: claimed export has required fields. Authenticity, signatures,")
    print("registry activation, actual subscriptions, contracts and production authority")
    print("MUST STILL be independently verified by A06/J05/affected consumer owners.")
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
