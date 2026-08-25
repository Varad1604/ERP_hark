# Closed Monitoring Case Reopening to Reinitialized Monitoring Cycle

# 1. Scenario Metadata

**Scenario ID:** L3-D-033  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Academic Monitoring  
**Participating Domains:** D  
**Scenario Type:** Critical  
**Priority:** P2  
**Status:** Review  
**Version:** 1.1  
**Owner:** Pallav  
**Last Updated:** 2026-08-25  
**Depends On:** `L2-D05_Academic_Monitoring_Workflows.md`  
**Governance Basis:** `L2-D05_Business_Architecture_Purity_Amendment_2026-08-24.md`  
**Referenced By:** None yet  
**Frozen Stage 3 Source:** P2W-D-0754 / CAN-L3-D-034 (final L3-D-033)

---

# 2. Scenario Purpose

Reopen a Closed Academic Monitoring Case through governance approval by creating a new lifecycle/evaluation context rather than rolling back or altering the original closure.

# 3. Business Outcome

An approved reopening creates a separately identified monitoring lifecycle with current policy, new plan and evaluation context while the original Closed lifecycle remains immutable.

# 4. Scenario Boundary and Scope

Starts when new evidence, policy/compliance change, governance decision or corrected interpretation justifies reopening; ends when a new monitoring cycle is initialized/activated and `NewMonitoringCycleStarted` is published with traceability to original closure.

In scope: reopening request, reason/impact review, approve/reject, create new lifecycle identity/context/plan and activate cycle. Out of scope: rollback of Closed case, modification of historical decisions/snapshot/evidence, technical rollback/retry mechanisms.

# 5. Ownership and Participating Domains

D05 owns reopening request, approval, new lifecycle identity, context, plan and activation.

# 6. Business Actors, Roles and Responsibilities

Monitoring Authority requests reopening and supplies reason/evidence; Governance Reviewer assesses reason/evidence/impact; Governance Authority Approves or Rejects reopening.

# 7. Preconditions and Entry Criteria

Original case is Closed; closure record, reopening reason/evidence and current applicable policy are available.

# 8. Trigger

A governed reopening condition is identified for a Closed monitoring case.

# 9. Main Business Flow

1. Monitoring Authority/D05 establishes reopening request referencing original closure — `MonitoringReopeningRequested`.
2. Governance Reviewer evaluates reason, evidence, compliance and impact.
3. Governance Authority Approves or Rejects — `MonitoringReopeningApproved` where approved; rejection retains original Closed state.
4. For approval, D05 creates a new lifecycle identity/context while preserving original closure — `MonitoringLifecycleReopened`.
5. D05 assigns current policy and creates new plan/evaluation context — `ReopenedMonitoringInitialized`.
6. D05/Monitoring Authority activates the new cycle — `NewMonitoringCycleStarted`.

# 10. Alternate Business Flows

## 10.1 Reopening Rejected

A valid review may end in rejection. The original case remains Closed and immutable; reason/evidence remain traceable.

# 11. Failure, Interruption and Business Recovery

Invalid/missing reopening evidence blocks review until resolved; invalid authority blocks decision; interruption preserves the last authoritative business position/evidence/context. The original Closed lifecycle is never rolled back.

# 12. Business Rules and Decision Constraints

Original Closed case remains immutable; reopening establishes a distinct new lifecycle; D05 owns both reopening and new cycle; repeated same business fact must not create duplicate institutional reopening outcomes. Technical rollback/idempotency/retry design is deferred per locked D05 Purity Amendment.

# 13. State and Lifecycle Impact

Original Academic Monitoring Case remains Closed/immutable. Reopened Monitoring Lifecycle: Created → Initialized → Active. Source: L2-D05 §§14.8–14.11.

# 14. Business Events and Cross-Domain Collaboration

D05 publishes `MonitoringReopeningRequested`, `MonitoringReopeningApproved`, `MonitoringLifecycleReopened`, `ReopenedMonitoringInitialized`, and `NewMonitoringCycleStarted` as defined in L2-D05 §§14.8–14.10.

# 15. Business Authorization, Approval and Audit Obligations

Monitoring Authority initiates; Governance Reviewer reviews; Governance Authority decides. Audit retains original closure reference, reason/evidence, review, authority/decision, new lifecycle identity, policy/plan/evaluation context and activation fact.

# 16. Completion Criteria and Postconditions

Approved reopening establishes a distinct active monitoring cycle; original closure remains immutable; D05 ownership remains unchanged.

# 17. Open Issues, Assumptions and Deferred Questions

No new open issue or assumption is introduced. Technical rollback, retries, persistence, transaction handling and deduplication are deferred to later levels.

---

**Draft Validation Verdict:** REVIEW REQUIRED  
**Validation Basis:** L2-D05; locked D05 Business Architecture Purity Amendment; Template v2.0; Architecture Level Validation Standards.
