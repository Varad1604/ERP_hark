# SutraERP L3 Phase 5 — Pallav Authority and Approval Traceability Remediation

**Date:** 2026-08-24  
**Branch:** `dhruv`  
**Architecture Level:** L3 — Business Scenario Architecture  
**Scope:** Pallav-assigned Phase 5 scenarios only  
**Purpose:** Record evidence-backed remediation of authority, locking, and approval-traceability defects without changing upstream business architecture.

---

## 1. Governing Basis

This remediation applies the existing locked L3 governance rather than creating new business architecture.

The locked L3 scenario template requires unresolved material ambiguity to be recorded in Section 17 and states that a material unresolved issue affecting authority, lifecycle, ownership, event publication, business rules, scenario boundary, or final-outcome correctness prevents a scenario from becoming `Locked`.

The locked Phase 5 parallel-authoring authorization defines the controlled status progression as:

`Draft -> Review -> Architecture Approved -> Locked`

and states that architecture approval and final locking remain governed review activities.

No statement in this remediation changes locked L1 ownership, L2 business rules, frozen Phase 4 scenario semantics, canonical scenario IDs, lifecycle states, business events, or canonical publishers.

---

## 2. Established Findings

### 2.1 L3-B-003 — Institution Archival

The scenario physically declared `Status: Approved` and `Architecture Lock: Locked` while Section 17 already recorded two material unresolved issues:

1. canonical actor / decision authority is incomplete or inconsistent in the applicable upstream material;
2. permitted source states for Institution archival are not fully enumerated upstream.

These issues affect authority and lifecycle correctness and therefore conflict with the locked L3 rule for final locking.

**Remediation:** return the scenario to `Review`, remove the operative lock marker, preserve the existing open issues, and make no local invention of actor authority or lifecycle entry states.

### 2.2 L3-B-030 — Calendar Exception to Completed Temporary Deviation

The scenario physically declared `Status: Approved` and `Architecture Lock: Locked`. Sections 6 and 15 explicitly state that the canonical actor / decision role is `UNCERTAIN` where unsupported upstream, while Section 17 incorrectly said `No open issues.`

The applicable L2-B05 source records the canonical `Actors` section as absent rather than defining the missing authority.

**Remediation:** return the scenario to `Review`, remove the operative lock marker, and record the authority gap in Section 17. No actor or approval role is invented at L3.

### 2.3 L3-G-016 — Facility Service Request to Operational Resolution

The scenario declared `Status: Architecture Approved` and cited historical approval reference `ACR-G-2026-08-21-002`, attributed to `Arjun Mehta, Chief Enterprise Architect`. A repository artifact establishing that exact ACR provenance was not located during this remediation.

The current approved L2-G01 source remains valid for the scenario's L2-supported business semantics; absence of historical L3 approval provenance does not, by itself, invalidate those semantics.

**Remediation:** return L3-G-016 to `Review`, preserve the historical approval claim as unverified traceability evidence rather than treating it as operative approval, and add an explicit Section 17 approval-provenance issue. Ownership, roles supported by L2-G01, states, events, contracts, and scenario boundaries remain unchanged.

### 2.4 Finance scenarios referencing `ACR-F-2026-08-21-001`

The following Pallav-assigned Draft scenarios were identified as referencing `ACR-F-2026-08-21-001` during the Phase 5 review:

- L3-F-034
- L3-F-035
- L3-F-036
- L3-F-037
- L3-F-038
- L3-F-067
- L3-F-068
- L3-F-069
- L3-F-070
- L3-F-071
- L3-F-072
- L3-F-080
- L3-F-081
- L3-F-082
- L3-F-083

No canonical repository artifact with that exact ACR identifier was located during the remediation. Current Finance upstream approval records provide useful approved L2 context but do not themselves establish the exact historical ACR identifier or every named role / segregation-of-duties assertion attributed solely to it.

These scenarios are already `Draft`; therefore no approval or lock downgrade is required in this remediation.

**Control:** any actor-authority, maker-checker, segregation-of-duties, or approval assertion that depends solely on `ACR-F-2026-08-21-001` remains unverified for approval purposes until it is tied to a canonical repository source. Independently L1/L2-supported ownership, states, events, rules, and cross-domain boundaries remain usable and must not be discarded merely because this traceability gap exists.

---

## 3. Applied Repository Corrections

| Scenario / Range | Correction | Architecture Effect |
|---|---|---|
| L3-B-003 | `Approved/Locked` -> `Review`; preserve existing open issues | Removes invalid lock while preserving upstream uncertainty |
| L3-B-030 | `Approved/Locked` -> `Review`; add missing Section 17 authority issue | Makes the scenario internally consistent with L2-B05 and locked L3 governance |
| L3-G-016 | `Architecture Approved` -> `Review`; historical ACR attribution marked unverified; add Section 17 provenance issue | Prevents unsupported approval provenance from operating as canonical approval |
| Listed Pallav Finance scenarios | No status change; remain `Draft`; ACR-F-dependent authority assertions are approval blockers until canonically sourced | Preserves sound Draft semantics while preventing unsupported authority claims from being promoted |

---

## 4. Explicit Non-Changes

This remediation does **not**:

- change any L1 canonical owner;
- change any L2 workflow, rule, lifecycle state, event, publisher, or contract;
- create a missing actor or approval role;
- create or backfill historical `ACR-F-2026-08-21-001` or `ACR-G-2026-08-21-002` evidence;
- change frozen Phase 4 canonical scenario outcomes or IDs;
- approve or lock any scenario;
- alter Pallav's human author allocation.

---

## 5. Remaining Upstream / Governance Actions

1. Resolve L3-B-003's archival decision authority and permitted archival entry states through the applicable authoritative upstream architecture.
2. Provide an authoritative actor / decision-authority definition for the L2-B05 Calendar Exception workflow if a named role is required at L3.
3. For Finance, locate a canonical source for the authority / segregation-of-duties assertions currently attributed solely to `ACR-F-2026-08-21-001`, or keep those assertions explicitly unresolved until such authority exists.
4. For Section G, either locate canonical provenance for the historical `ACR-G-2026-08-21-002` approval claim or perform a governed current approval after scenario review. Historical provenance must not be fabricated.
5. Run a current-branch sweep for other scenarios that combine an unresolved material issue with `Locked` or `Architecture Approved` metadata and synchronize them under the same locked governance rules.

---

## 6. Architecture Integrity Statement

The corrections in this record are governance-alignment changes only. They reduce false certainty and restore traceability without changing canonical business ownership or business behaviour. Unresolved upstream decisions remain unresolved and visible; they are not silently solved at L3.
