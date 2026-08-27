# SutraERP L4 Governance Baseline Lock Decision

**Version:** 1.0.0  
**Status:** Locked  
**Owner:** Lead Architect  
**Architectural Level:** L4 — System Behavior Specification Governance  
**Domain:** Cross-domain  
**Last Updated:** 2026-08-26  
**Depends On:** L4 Controlled Pilot Validation Report; Repository Constitution Volume 2 v1.2; L4 Governance and Architecture Standard; L4 System Behavior Specification Template v1.0; L4 Validation Standard  
**Referenced By:** L4 README; future L4 authoring authorization and execution plans  

---

## 1. Decision

The SutraERP L4 governance framework is approved and locked as the canonical baseline for future Level 4 System Behavior Specification authoring.

This decision locks the **governance method**, not the L4 corpus.

---

## 2. Locked Decisions

The following decisions are now canonical:

1. **L4 Definition**  
   L4 defines mandatory, observable, implementation-independent system behavior required to realize an approved L3 Business Scenario.

2. **Parent Eligibility**  
   Canonical L4 authoring requires a current L3 parent at `Architecture Approved` or `Locked` that passes the L4 parent-readiness gate.

3. **Parent Naming**  
   Parent L4 IDs inherit the approved L3 scenario section and number:

   ```text
   L3-D-007 → L4-D-007
   ```

4. **Filename Naming**  
   Canonical filename:

   ```text
   L4-D-007_<Canonical_L3_Scenario_Slug>.md
   ```

5. **Behavioral Obligation Naming**  
   Atomic System Behavioral Obligations use:

   ```text
   L4-D-007-BH-001
   ```

6. **Physical Granularity**  
   Default is one eligible L3 scenario → one parent L4 specification → multiple BHs inside the file.

7. **Physical Split Exception**  
   A split is permitted only by controlled architecture-review exception where the parent is materially unreviewable; all split artifacts must retain one parent traceability identity and must not create duplicate ownership.

8. **Canonical Template**  
   The locked 10-section `L4_System_Behavior_Specification_Template_v1.0.md` is canonical.

9. **Canonical Validation**  
   `L4_Validation_Standard.md` defines parent readiness, BH admission, hard failures, review verdicts, upstream-change revalidation and corpus-level lock gates.

10. **Legacy Template**  
    `templates/L4_SystemBehavior_Template.md` is `Deprecated` and shall not be used for new L4 authoring.

11. **Traceability**  
    Every BH must have one or more material L3 provenance anchors. L2/L1/L0 may be used selectively to verify exact semantics but not to silently repair material L3 omissions.

12. **Architecture Boundary**  
    APIs, database/persistence design, services/classes/components, queues/brokers, retry/transaction/rollback mechanics, infrastructure/technology, technical IAM/RBAC and UI/UX remain outside L4.

13. **Upstream Change Propagation**  
    Material upstream changes require impact assessment and re-review of affected L4 specifications before continued approval/lock.

---

## 3. Pilot Evidence

The controlled pilot validated the method across:

- `L3-D-007` — cross-domain waitlist resolution;
- `L3-I-007` — examination planning/approval;
- `L3-I-118` — revaluation/revised marks outcome.

The pilot demonstrated:

- BH-level cohesion and reviewability;
- strong future L5 traceability;
- successful preservation of ownership and event boundaries;
- no need for a 17-section L4 template;
- no need to split BHs into individual files;
- the necessity of a mandatory parent-readiness gate;
- the necessity of observable-behavior admission rather than governance-prose BHs.

---

## 4. Explicitly Not Locked by This Decision

The following remain outside this governance lock decision:

- the total number of L4 files in the final corpus;
- team workload allocation;
- complexity weights;
- parallel-author batch size;
- human author assignments;
- final eligible L3 authoring population;
- final L4 corpus completion/lock;
- L5 architecture decisions.

The default one-L3-to-one-L4 model is locked as the normal granularity rule, but the final physical file count cannot be known until the eligible L3 baseline and any approved split/exclusion dispositions are validated.

---

## 5. Broad Authoring Gate

Broad or parallel L4 authoring is **not authorized by this decision alone**.

Before broad authoring begins, governance must confirm:

1. current repository-wide eligible L3 parent baseline;
2. current Architecture Index/navigation consistency;
3. canonical production L4 folder structure and pilot separation;
4. author/reviewer role assignments;
5. execution/batch controls.

Only then may a separate L4 parallel-authoring authorization be issued.

---

## 6. Repository Impact

This decision is implemented through:

- Repository Constitution Volume 2 v1.2;
- `L4_Governance_and_Architecture_Standard.md`;
- `L4_System_Behavior_Specification_Template_v1.0.md`;
- `L4_Validation_Standard.md`;
- L4 README;
- deprecation of `templates/L4_SystemBehavior_Template.md`;
- Architecture Level Validation Standards v1.2.

No L1/L2/L3 business ownership, rule, state, event, authority or scenario identity is changed by this governance lock.

---

## 7. Final Decision

**L4 GOVERNANCE BASELINE: LOCKED**

**L4 MASS AUTHORING: NOT YET AUTHORIZED**

The next controlled stage is repository-wide L3 eligibility/baseline validation and L4 execution preparation.
