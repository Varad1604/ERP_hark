# Credit Mapping Rule Configuration

# 1. Scenario Metadata

**Scenario ID:** L3-D-046  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Open Electives & MOOCs  
**Participating Domains:** B, C, D  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.1  
**Owner:** Pallav  
**Last Updated:** 2026-08-25  
**Depends On:** `L2-D06_Open_Electives_and_MOOCs_Workflows.md`  
**Frozen Stage 3 Source:** P2W-D-0766 / CAN-L3-D-046

---

# 2. Scenario Purpose

Define, validate, approve and publish versioned institutional equivalency rules between flexible/external learning and canonical curriculum.

# 3. Business Outcome

One governed Active mapping exists for the applicable curriculum version, is available to future Credit Recognition cases and does not retroactively alter completed recognition decisions.

# 4. Scenario Boundary and Scope

Starts when curriculum/new opportunity/equivalency-governance change requires a mapping version; ends when `CreditMappingRulesPublished` is published for the governed mapping. In scope: canonical curriculum/outcome references, equivalency/credit/scope/effective period, duplicate/limit/policy validation, review/approval/publication and predecessor history. Out of scope: individual recognition decision, transcript posting, technical rules engine.

# 5. Ownership and Participating Domains

D06 owns Credit Mapping versions/equivalency configuration. Section B/Curriculum owns curriculum/course/Learning Outcome definitions. Section C/Academic Records owns permanent student academic records/transcript facts.

# 6. Business Actors, Roles and Responsibilities

Curriculum Committee evaluates academic equivalency/credit values; Academic Affairs prepares/validates mapping. The exact canonical final mapping-approval role is not safely established by the reviewed L3 evidence and is therefore recorded as an upstream authority question rather than named generically.

# 7. Preconditions and Entry Criteria

Canonical curriculum version is Published; candidate learning opportunity/provider evidence and applicable approval governance are available.

# 8. Trigger

Curriculum/equivalency policy change requires a new mapping version.

# 9. Main Business Flow

1. D06 establishes mapping version against canonical curriculum/opportunity.
2. Curriculum Committee evaluates outcome equivalency, credit values and permitted recognition scope.
3. D06 validates duplicate mapping, curriculum availability, credit limits and policy.
4. Applicable institutional mapping authority reviews/approves or rejects.
5. For approval, D06 publishes the governed Active mapping — `CreditMappingRulesPublished`.
6. D06 preserves prior versions and applies the new version only to future recognition cases as governed.

# 10. Alternate Business Flows

No alternate route reaches Published if validation/approval fails; rejection/return belongs in Section 11.

# 11. Failure, Interruption and Business Recovery

Invalid curriculum/policy/duplicate/limit condition blocks publication until corrected. Missing/invalid approval authority retains prior Active mapping. Interrupted mapping progression preserves last authoritative D06 mapping position/version.

# 12. Business Rules and Decision Constraints

D06 owns mapping version; Section B owns curriculum definitions; completed recognition decisions are not retroactively changed; only one governed Active mapping applies to the relevant version/scope where L2-D06 requires it; technical rules-engine behaviour is deferred.

# 13. State and Lifecycle Impact

Credit Mapping Rule: Draft → Validated → Approved → Published/Active; prior version retained. Owner D06. Source L2-D06 W62/ADR-007/008.

# 14. Business Events and Cross-Domain Collaboration

D06 publishes `CreditMappingRulesPublished` under L2-D06 W62/D06-E53. Section B/Academic Records supply/retain their own canonical facts.

# 15. Business Authorization, Approval and Audit Obligations

Curriculum Committee/Academic Affairs act within established academic responsibility. Exact final publication authority must be confirmed upstream. Audit retains mapping version, source curriculum references, equivalency rationale, credit values/scope, validation, approval and version lineage.

# 16. Completion Criteria and Postconditions

Governed mapping is Published/Active for future recognition cases; historical recognition decisions remain unchanged; ownership remains separated.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

Exact canonical institutional business-role name for final Credit Mapping publication approval requires upstream confirmation; generic `Authorized academic governance authority` is removed.

## 17.2 Assumptions

No assumption overrides locked architecture.

## 17.3 Deferred Questions

Rules-engine implementation, commands/APIs and technical RBAC are deferred to later levels.

---

**Draft Validation Verdict:** REVIEW REQUIRED  
**Validation Basis:** L2-D06; Template v2.0; Architecture Level Validation Standards.
