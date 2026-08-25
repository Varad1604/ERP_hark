# External Learning Record Reconciliation

# 1. Scenario Metadata

**Scenario ID:** L3-D-049  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Open Electives & MOOCs  
**Participating Domains:** D  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Review  
**Version:** 1.1  
**Owner:** Pallav  
**Last Updated:** 2026-08-25  
**Depends On:** `L2-D06_Open_Electives_and_MOOCs_Workflows.md`  
**Frozen Stage 3 Source:** P2W-D-0769 / CAN-L3-D-049

---

# 2. Scenario Purpose

Reconcile discrepancies between canonical institutional D06 learning records and external-provider records while preserving institutional ownership and complete evidence.

# 3. Business Outcome

Each discrepancy receives a governed reconciliation outcome, institutional D06 truth remains authoritative, and `LearningRecordsReconciled` is published without destructive overwrite.

# 4. Scenario Boundary and Scope

Starts when comparison/synchronization/audit identifies a discrepancy; ends when reconciliation outcome/evidence are recorded and institutional D06 records are consistent. In scope: compare identity/status/progress/assessment/completion facts, classify discrepancy, apply approved business reconciliation, request provider-side correction where needed, escalate identity/fraud/unresolved inconsistency. Out of scope: treating provider data as automatically canonical, destructive overwrite, reconciliation service/algorithm/retry design.

# 5. Ownership and Participating Domains

D06 owns institutional external-learning registration/progress/completion/reconciliation outcome. External Learning Provider owns provider-side records supplied as evidence.

# 6. Business Actors, Roles and Responsibilities

Academic Affairs approves academic interpretation where discrepancy affects completion/recognition. Compliance Office acts where identity/fraud investigation is required. Comparison/reconciliation execution is a D06 business responsibility, not an `Integration Administrator` or `Reconciliation Service` actor.

# 7. Preconditions and Entry Criteria

Correlated institutional/provider identifiers and comparison evidence exist; applicable reconciliation policy and authoritative D06 position are available.

# 8. Trigger

Comparison, audit or provider/institution review detects a learning-record discrepancy.

# 9. Main Business Flow

1. D06 establishes reconciliation case/evidence.
2. D06 compares registration/progress/assessment/completion/identity facts.
3. D06 confirms authoritative institutional position and applicable policy/version.
4. D06 applies permitted institutional reconciliation or requests provider-side correction while preserving history.
5. Academic Affairs/Compliance handles academic/fraud/identity exceptions where required.
6. D06 validates final consistency and publishes `LearningRecordsReconciled`.

# 10. Alternate Business Flows

No alternate route reaches reconciled closure while an unresolved identity/fraud/historical inconsistency remains; such conditions belong in Section 11.

# 11. Failure, Interruption and Business Recovery

Unresolved identity/provider evidence blocks reconciliation; invalid authority blocks exceptional interpretation; interruption preserves last authoritative institutional/provider comparison position and evidence. Provider-side disagreement follows governed escalation/reconciliation rather than destructive overwrite.

# 12. Business Rules and Decision Constraints

Institutional D06 truth remains authoritative for D06-owned records; provider owns provider-side records; history is preserved; technical synchronization/retry/deduplication is deferred.

# 13. State and Lifecycle Impact

Learning Reconciliation Case: Discrepancy Detected → Resolved or Escalated with evidence. Owner D06. Source L2-D06 W70–W71.

# 14. Business Events and Cross-Domain Collaboration

D06 retains canonical events `ProviderSynchronizationRecovered`, `ProviderSynchronizationRecoveryFailed`, and `LearningRecordsReconciled` per L2-D06 W70–W71/Event Catalogue. Event implementation mechanics are deferred.

# 15. Business Authorization, Approval and Audit Obligations

Academic Affairs/Compliance act only within policy authority. Audit retains both evidence sets, authoritative D06 position, reconciliation policy, decision authority/reason, provider correction request where applicable and final reconciliation outcome.

# 16. Completion Criteria and Postconditions

Discrepancy has governed outcome; D06 institutional truth is consistent; provider ownership remains separate; no historical evidence is destroyed.

# 17. Open Issues, Assumptions and Deferred Questions

No new open issue/assumption. Reconciliation algorithms, synchronization transport, retries and persistence are deferred.

---

**Draft Validation Verdict:** REVIEW REQUIRED  
**Validation Basis:** L2-D06; Template v2.0; Architecture Level Validation Standards.
