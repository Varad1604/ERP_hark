# L3 Section H — Audit Correction and Phase 2 Traceability Register

**Architecture Level:** L3 — Business Scenario Architecture  
**Applies To:** Section H — Institutional Services & Intelligence  
**Current candidates:** `CAN-L3-H-001` … `CAN-L3-H-081` (renumbered 2026-08-17, see `L3_Phase1_Candidate_ID_Migration_2026-08-17.md`)  
**Status:** Active — corrected Phase 1 input  
**Created:** 2026-08-12  
**Authority:** user-approved resolution of `generated/reviews/L3_Phase1_Audit_Varad_ALL_ASSIGNED.md`

---

## 1. Purpose

This register preserves traceability for the audit correction of the former Section H tail in `L3_Phase1_Inventory_Varad.md`. The former material was not safe Phase 1 input: it used nonconforming IDs, contained malformed rows and invalid classification values, represented lifecycle steps as separate L3 scenarios, and placed a 238-row cross-section summary under H09.

The corrected inventory uses institutional business outcomes. Detailed rules, validation branches, state transitions, event emissions, audit controls and read-model mechanics remain authoritative in their original L2 sources and are candidates for L4 behavior specifications; their removal as standalone L3 rows does not remove the underlying architecture.

---

## 2. Accepted Architecture Decisions

| Decision | Accepted boundary | Repository application |
| --- | --- | --- |
| `H-DEC-01` — Dashboard ownership | H07 owns dashboard composition, templates, widgets, layouts, workspaces and presentation governance. Finance and Campus Operations retain ownership of source facts, reports, KPI definitions and projections. | Applied to L2-F13, the Dependency Map, and affected F/G/H candidates. |
| `H-DEC-02` — External credit recognition | L2-C04 owns academic equivalency, approval/rejection, recognized credit and the Academic Credit Ledger. H10 owns provider verification, external exchange, mapping, synchronization, recovery and reconciliation. | Applied to L2-C04, L2-H10, the Dependency Map, `CAN-L3-C-032`, and `CAN-L3-H-078`. |
| `H-DEC-03` — Complete H09 before L3 | H09 must define its integration contracts, dependency model and locked decisions before accreditation/compliance L3 candidates can be approved. | L2-H09 sections 23–25 added before H09 candidates were corrected. |
| `H-DEC-04` — Separate readiness outcomes | A10 owns operational go-live/certification eligibility. G00 owns Campus Operations evidence readiness. H09 owns institutional accreditation readiness. | Applied to L2-G00, L2-H09, the Dependency Map, `CAN-L3-G-003`, and `CAN-L3-H-069`. |

These are user-authorized architecture corrections dated 2026-08-12. They do not transfer source-business ownership to a cross-cutting service.

---

## 3. Former-to-Current Candidate Crosswalk

| Former inventory block | Former rows | Current candidates | Disposition |
| --- | ---: | --- | --- |
| `CAN-L3-H01-001` … `CAN-L3-H01-011` | 11 | `CAN-L3-H-001` … `H-094` | Consolidated around linking, consent, publication, correction/revocation, reconciliation, recovery and connector outcomes. |
| `CAN-L3-H02-001` … `CAN-L3-H02-086` | 86 | `CAN-L3-H-010` … `H-101` | Delivery steps, channel attempts, schedules, receipts, retries and template transitions absorbed into seven end-to-end communication outcomes. |
| `CAN-L3-H03-087` … `CAN-L3-H03-212` | 126 | `CAN-L3-H-017` … `H-106` | Query, filtering, ranking, indexing, projection and recovery steps consolidated into enterprise discovery outcomes. |
| `CAN-L3-H04-214` … `CAN-L3-H04-263` | 50 | `CAN-L3-H-022` … `H-113` | Placement steps consolidated into employer, opportunity, application/selection, offer/joining, internship, exception and reporting journeys. Former suffix `213` was absent; current numbering removes the gap. |
| `CAN-L3-H05-264` … `CAN-L3-H05-313` | 50 | `CAN-L3-H-029` … `H-123` | Proposal, project, grant, ethics, publication, IP, collaboration, consultancy and portfolio steps consolidated. Proposal withdrawal is now explicit in `H-115`. |
| `CAN-L3-H06-314` … `CAN-L3-H06-363` | 50 | `CAN-L3-H-039` … `H-132` | Alumni transition, membership, profile, chapter, networking, mentoring, contribution, recognition and donation outcomes retained. |
| `CAN-L3-H07-364` … `CAN-L3-H07-398` | 35 | `CAN-L3-H-048` … `H-058` | Former five-column rows were replaced with governed dashboard-composition outcomes carrying all eight required fields. |
| `CAN-L3-H08-399` … `CAN-L3-H08-453` | 55 | `CAN-L3-H-055` … `H-145` | Regulatory framework, cycle, collection, approval, submission, correction and closure steps consolidated. |
| `CAN-L3-H09-454` … `CAN-L3-H09-691` | 238 | `CAN-L3-H-061` … `H-156` | The former block was a pasted fixed-width A–J summary. Only former `H09-656` directly represented H09 evidence collection. The corrected set represents all material H09 outcomes from the completed L2 source. |
| `CAN-L3-H10-692` … `CAN-L3-H10-883` | 192 | `CAN-L3-H-072` … `H-166` | Provider, connector, trust, synchronization, enrollment, learning, credit, verification, reconciliation and exchange steps consolidated. |

Former Section H total: **893 rows**. Current Section H total: **85 candidates**.

---

## 4. Explicit Additions and Removals

### Added material outcomes

- `CAN-L3-H-030` — submitted research proposal withdrawal.
- `CAN-L3-H-061` … `CAN-L3-H-071` — framework, cycle planning, compliance requirement, evidence, internal audit, external assessment, CAPA, quality improvement, accreditation readiness, compliance review and Self-Study Report handoff.
- `CAN-L3-H-078` — verified external credit request through synchronization of the authoritative L2-C04 academic decision.

### Removed unsupported concept

- Former `CAN-L3-H10-699` — **Reinstate External Provider**. L2-H10 defines retirement as terminal and contains no provider reinstatement transition. A retired provider must complete a new governed onboarding if the relationship is later required.

### Removed as standalone L3 candidates, retained as lower-level architecture

- Individual validations, commands, queries, queue steps, audit writes, notification sends, event-publication steps and read-model refresh mechanics.
- Alternate rejection, retry, cancellation, expiry, suspension, resumption, correction, reversal, reopening, reconciliation and archival behavior is retained inside the relevant scenario boundary where L2 supports it.

---

## 5. Cross-Inventory Relationships

| ID | Candidates | Relationship | Status |
| --- | --- | --- | --- |
| `H-XI-01` | `CAN-L3-H-048` … `H-058` with F dashboard rows and `CAN-L3-G-305`, `G-368` | Valid Phase 1 overlap. H07 is the final dashboard-composition owner; F/G supply authoritative reports, KPIs and projections. | `DETERMINED` — consolidate on H07 boundaries in Phase 2. |
| `H-XI-02` | `CAN-L3-C-032` and `CAN-L3-H-078` | Two consecutive but separately owned outcomes: C04 makes and records the academic decision; H10 synchronizes an external request. | `CLOSED` — retain both and bind by decision reference. |
| `H-XI-03` | `CAN-L3-G-003` and `CAN-L3-H-069` | G00 produces Campus Operations evidence readiness; H09 makes the institutional accreditation-readiness decision. | `CLOSED` — retain both as an evidence handoff. |
| `H-XI-04` | `CAN-L3-A-042` and `CAN-L3-H-069` | Operational certification eligibility and accreditation readiness are distinct outcomes. INT-302 was resolved on 2026-08-13: A10 owns persisted governance/certification while L1-06 owns dynamic capability readiness. That boundary does not transfer accreditation ownership from H09. | `CLOSED` for ownership; INT-302 is resolved. |

---

## 6. Remaining Phase 2 Work

- Reconcile the intentionally retained H07/F/G dashboard discovery overlap onto H07 final-outcome boundaries.
- Rebind detailed L4 acceptance criteria from the former Section H rows to the 85 current candidate IDs.
- Perform the repository's required Phase 2 review of provisional Type and Priority values.
- Do not reopen the four ownership decisions above unless a higher-authority locked repository decision changes.

Sections F and G have independent consolidation watchlists. Their broader fragmentation backlogs were not silently resolved by this Section H correction.

---

## 7. Validation Rules

- Current Section H IDs must match `CAN-L3-H-NNN` and remain unique.
- Every current row must contain exactly the eight Phase 1 fields.
- `Type` must be one of `Simple`, `Cross-domain`, or `Critical`.
- `Priority` must be one of `P0`, `P1`, or `P2`.
- `Domains` must contain SutraERP section letters only; external providers are actors, not domains.
- H07 may compose dashboards but must not compute or own source-domain KPIs.
- H10 may synchronize L2-C04 credit decisions but must not publish an H10-owned academic approval or rejection.
- H09 may reference evidence but must not copy source-domain records into a competing canonical store.


---

# 2026-08-15 H03 / H07 Boundary Ruling

Applied under Lead Architect approval. Record:
`generated/reviews/L3_Phase1_Tranche3_Application_Report_2026-08-15.md`.

Section H capability ownership prevails for composition and governed-availability outcomes.
Rows describing *use* of a capability, or personal presentation state, sit below Phase 1
altitude and are retired.

| Decision | Candidates |
|---|---|
| Retired — use of the capability, or personal presentation state | `H-017`, `H-019`, `H-053` |
| Retired — source-driven refresh is mechanism (precedent `G-FR-12`) | `H-058` |
| Retained — governed composition, versioning and widget availability | `H-133`, `H-137`, `H-139` |
| Retained — governed onboarding of a domain into enterprise search | `H-105` |
| Retained under this ruling, flagged for Phase 2 | `H-103` |

**Open point.** `H-103` (search projection currency) and the retired `H-058` (dashboard
refresh) are the same construct applied to two capabilities. `H-103` was retained because the
approved scope named it, but the `G-FR-12` precedent arguably retires it too. Phase 2 should
settle H-103 explicitly rather than leave the asymmetry standing.

`H-104` and `H-106` were re-titled to `Enterprise Search Index Failure to Verified Recovery`
and `Enterprise Search Configuration Change to Effective Governed Behavior`. They were never
action fragments — both were already noun-phrase outcome titles, and `check_l3_inventory.py`
was matching the noun "Search" against its `FRAGMENT_VERBS` list. The prefix removes the
ambiguity without changing meaning. Section H: 85 -> 81 candidates.
