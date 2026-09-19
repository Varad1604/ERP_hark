# SutraERP L6 — Experience Slice Register — Draft

**Version:** 0.1.0  
**Status:** Draft / Supporting Delivery Register  
**Owner:** Lead Architect  
**Architecture Level:** L6 — UI / UX Architecture Support  
**Last Updated:** 2026-09-18  
**Purpose:** Locate selected delivery experience slices and their evidence without replacing workspace-family coverage, approved L6 specifications, or canonical L4→L5 traceability.

> An experience slice is a bounded, traceable increment in which a specified user, in a specified context, can achieve one meaningful observable outcome—or reach a clearly defined handoff—with the material alternate, failure, accessibility and responsive behavior reviewable.

---

## 1. Authority and Non-Goals

This register is **not**:

- a new architecture level;
- a final ERP screen count;
- a business-workflow inventory;
- a replacement for the 99 business workspace families + 6 shared L6 surfaces;
- a replacement for the canonical L3/L4 corpus;
- a duplicate BH→L5 relation store;
- a new approval authority;
- a requirement to create one route, component or folder per row.

The controlling order remains:

```text
approved upstream architecture
→ approved L6 package / proceed-gate decision
→ selected delivery experience slice
→ implementation + review evidence
```

A delivery experience slice may inherit an existing approved L6 package boundary. Component refactors, fixtures, review cases and implementation-local refinements do not require a new formal architecture approval unless they materially change approved L6 semantics.

---

## 2. Relationship to Existing L6 Planning

- `L6_Screen_and_Workspace_Inventory_DRAFT.md` provides **architecture coverage** through 99 business workspace families + 6 shared surfaces.
- `L6_Persona_Journey_and_Navigation_Map_DRAFT.md` provides **user-goal and navigation composition**, including journeys that may cross workspace families.
- `L6_Screen_Journey_Traceability_Register_DRAFT.md` provides **workspace-family-to-upstream traceability and readiness state**.
- this register provides a **thin delivery locator** for selected reviewable increments;
- `02_Implementation_Review/` provides executable review cases and finding evidence;
- frontend code provides implementation evidence only.

---

## 3. Register Fields

Each selected row records:

- provisional delivery slice key and title;
- user outcome and scope boundary;
- primary workspace/shared surface;
- related workspace families;
- journey reference;
- presentation profile and operating context;
- controlling L6 specification and approval;
- exact upstream trace-binding reference;
- integration maturity;
- review-pack reference;
- implementation/test references;
- delivery/review status;
- responsible owners;
- dependencies, exclusions and unresolved issues.

Keys in this register are provisional delivery identifiers only. They are **not permanent L6 specification IDs**.

---

## 4. Selected W-C02 Delivery Experience Slices

Controlling package for every row below:

- L6 architecture: `L6_Student_Workspace_DRAFT.md`;
- controlling approval: `00_Governance/L6_Student_Workspace_Validation_and_Proceed_Gate_Decision_2026-09-17.md`;
- approved parent set: `L3/L4-C-001`, `L3/L4-C-008…019`;
- canonical trace evidence: W-C02 L4 parent approval + canonical Section-C BH→L5 relation store + `L5_W-C02_Traceability_Validation_Decision_2026-09-17.md`;
- review pack: `02_Implementation_Review/W-C02_Student_Workspace_Review_Pack.md`.

| Provisional key | Delivery experience slice | User outcome / completion boundary | Primary workspace | Related workspace families | Journey ref | Profile / context | Upstream scenario binding | Integration maturity | Implementation / test evidence | Delivery status | Owners | Dependencies / exclusions / unresolved issues |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| `ES-W-C02-01` | Staff orientation and Student record inspection | Authorized staff can enter a Student context, understand current Student-owned context, inspect record/history and identify source-owned projections without losing subject/context | W-C02 | W-C01 context only where approved; source-owned projection destinations | J1, J2 | Student Operations / Institutional Admin; confirmed Institution + academic context | C-001, C-008, C-009 plus read-only W-C02 projection obligations | Fixture-backed presentation; released read integration not claimed | Frontend pilot branch; review cases `W-C02-RC-01/02` | Frontend implemented with deterministic presentation cases; browser acceptance pending; released-contract integration not claimed | L6 Lead Architect / Frontend implementation | No cross-domain edit authority; lifecycle history remains distinct from canonical audit evidence |
| `ES-W-C02-02` | Student self-orientation | Student can understand own details, academic context, requests/documents and related statuses without staff lifecycle controls | W-C02 | source-owned projection destinations | J6 | Student self-service; current authenticated Student context | W-C02 self-service composition across approved C-008…019 projections where applicable | Fixture-backed presentation; real authorization/session enforcement not claimed | Frontend pilot branch; review case `W-C02-RC-03` | Frontend implemented with deterministic presentation cases; browser acceptance pending; released-contract integration not claimed | L6 Lead Architect / Frontend implementation | Persona composition is not authorization; no parallel Student model |
| `ES-W-C02-03` | Document-request intent and follow-through | User can initiate a supported request presentation, understand submission/request/result distinctions, return later and access a result only when source-supported | W-C02 | Document Services technical capability where applicable; no business ownership transfer | J5 | Student or authorized staff where upstream capability permits | C-018 | Presentation + deterministic review fixtures only until released request contract/required inputs are evidenced | Frontend pilot branch; review cases `W-C02-RC-04…09` | Frontend implemented with deterministic presentation cases; browser acceptance pending; released-contract integration pending | L6 Lead Architect / Frontend implementation | Do not invent payload, required fields, issuance authority or rendering=issuance semantics |
| `ES-W-C02-04` | Related-source projection and owner-preserving handoff | User can inspect one source-owned projection, understand ownership, continue toward the owner or an explicitly unavailable preview, and return without losing Student context | W-C02 | Finance selected for pilot; future Registration/Examination/Attendance handoffs remain separate review work | J4 | Authorized staff / Student where projection exposure is supported | W-C02 read-only cross-domain projection obligations; source owner remains Finance | Fixture-backed projection; owner destination may be bounded preview until approved implementation exists | Frontend pilot branch; review cases `W-C02-RC-10/11` | Frontend implemented with deterministic presentation cases; bounded owner preview available; owning-destination integration pending | L6 Lead Architect / Frontend implementation | No Finance mutation in W-C02; unavailable owner destination must not be presented as end-to-end completion |
| `ES-W-C02-05` | Suspension review and controlled action seam | Authorized staff can identify suspension, understand current Student/context and source-supported consequence, review the action, and observe pending/uncertain/rejected presentation without local lifecycle transition | W-C02 | none | J3 | Authorized staff presentation; current Student + confirmed protected context | C-010 | Presentation seam + deterministic review fixtures only; released mutation contract/eligibility not claimed | Frontend pilot branch; review cases `W-C02-RC-12…16` | Frontend implemented with deterministic presentation cases; browser acceptance pending; authoritative mutation integration pending | L6 Lead Architect / Frontend implementation | No local eligibility, approval policy or transition; duplicate/repeat safety must remain explicit for indeterminate outcomes |
| `ES-W-C02-06` | Profile and communication-preference maintenance boundary | User can review current Student-owned profile/contact context and a supported maintenance entry without the client inventing required fields or identity authority | W-C02 | Person & Identity boundary where applicable | J2/J6 | Authorized staff / Student self-service where permitted | C-014, C-015 | Presentation fixture; released mutation/input contract not claimed | Frontend branch; review cases `W-C02-RC-19…21` | Frontend implemented with deterministic presentation cases; browser acceptance pending; released-contract integration pending | L6 Lead Architect / Frontend implementation | No Person ownership transfer; exact production fields/options remain contract-owned |
| `ES-W-C02-07` | Enrollment and academic-assignment maintenance boundary | Authorized staff can review current enrollment/assignment context and start a governed maintenance flow without locally deciding validity | W-C02 | Academic Registration/Progression remain separate owners where applicable | J3 | Authorized staff | C-008, C-009 | Presentation fixture; released mutation contract not claimed | Frontend branch; review cases `W-C02-RC-22…24` | Frontend implemented with deterministic presentation cases; browser acceptance pending; released-contract integration pending | L6 Lead Architect / Frontend implementation | No Registration/Progression mutation inside W-C02; exact validation remains upstream |
| `ES-W-C02-08` | Restoration after suspension | Authorized staff can review restoration consequence and observe governed outcome states without inferring eligibility or transition | W-C02 | none | J3 | Authorized staff; suspended context fixture only | C-011 | Presentation fixture; authoritative mutation not connected | Frontend branch; review cases `W-C02-RC-25…28` | Frontend implemented with deterministic presentation cases; browser acceptance pending; released-contract integration pending | L6 Lead Architect / Frontend implementation | Eligibility and lifecycle transition remain authoritative upstream |
| `ES-W-C02-09` | Intra-Institute academic reassignment | Authorized staff can review current placement, proposed fixture destination and governed outcome without collapsing it into inter-Institute transfer | W-C02 | Academic structures as projection only | J3 | Authorized staff | C-012 | Presentation fixture; mutation contract not connected | Frontend branch; review cases `W-C02-RC-29…32` | Frontend implemented with deterministic presentation cases; browser acceptance pending; released-contract integration pending | L6 Lead Architect / Frontend implementation | C-012 remains distinct from C-013; no local validity decision |
| `ES-W-C02-10` | Inter-Institute transfer and context re-establishment | Authorized staff can review transfer consequence/context-reset boundary without invented approving actor or completion event | W-C02 | destination Institution context only where supported | J3 | Authorized staff | C-013 | Presentation fixture; mutation/approval integration not connected | Frontend branch; review cases `W-C02-RC-33…36` | Frontend implemented with deterministic presentation cases; browser acceptance pending; released-contract integration pending | L6 Lead Architect / Frontend implementation | No invented approving actor, completion event or automatic context switch |
| `ES-W-C02-11` | Identity correction boundary | Authorized user can review Student-side identity correction entry and evidence-sensitive boundary without replacing canonical Person identity | W-C02 | W-A02 Person & Identity | J3 | Authorized specialist/staff presentation where supported | C-016 | Presentation fixture; evidence/approval integration not connected | Frontend branch; review cases `W-C02-RC-37…40` | Frontend implemented with deterministic presentation cases; browser acceptance pending; released-contract integration pending | L6 Lead Architect / Frontend implementation | No Person ownership transfer; exact evidence/approval remains upstream |
| `ES-W-C02-12` | Duplicate Student lifecycle resolution boundary | Authorized user can review duplicate-resolution context/consequence while preserving upstream merge semantics | W-C02 | W-A02 Person & Identity | J3 | Authorized specialist/staff presentation where supported | C-017 | Presentation fixture; resolution integration not connected | Frontend branch; review cases `W-C02-RC-41…44` | Frontend implemented with deterministic presentation cases; browser acceptance pending; released-contract integration pending | L6 Lead Architect / Frontend implementation | No client-side merge; no invented Student↔Person merge sequencing |
| `ES-W-C02-13` | Closure/archive and preserved history | Authorized staff can review permanent closure consequence and preserved-history boundary without treating archival as deletion | W-C02 | audit/evidence remains separate | J3 | Authorized staff | C-019 | Presentation fixture; authoritative closure integration not connected | Frontend branch; review cases `W-C02-RC-45…48` | Frontend implemented with deterministic presentation cases; browser acceptance pending; released-contract integration pending | L6 Lead Architect / Frontend implementation | Archival ≠ physical deletion; human-readable history ≠ canonical audit evidence |


---

## 5. W-C02 Continuation Interpretation

The rows above remain **delivery planning boundaries**, not new business workflows and not a locked final count. They may be consolidated or refined later if implementation evidence shows a stronger UX boundary without changing canonical scenario ownership.

C-012 and C-013 may share presentation components but remain distinct business scenarios. C-013 receives no invented approving actor or completion event. C-016/C-017 receive no invented Person-merge sequencing.

Additional source-owned handoff cases may be added as owning destinations become approved/implemented; these do not transfer ownership into W-C02.

---

## 6. Status Dimensions

Do not collapse these into one “complete” flag:

1. **Architecture authorization** — whether the controlling L6 package is approved.
2. **Frontend implementation** — whether the delivery increment is represented in code.
3. **Fixture-based UX acceptance** — whether deterministic review cases pass.
4. **Released-contract integration** — whether the real backend/application contract is implemented.
5. **Integrated production validation** — whether the connected production-like journey has been validated.

`Workspace represented` does not mean `workspace complete`.  
`Fixture journey passed` does not mean `production ERP ready`.


---

## 7. Relationship to Full Workflow Realization Expansion

The full workflow realization programme does not turn this register into a 1,084-row scenario register.

Roles remain distinct:

- `L6_Workflow_Realization_Register_DRAFT.md` records the primary UI realization disposition for every canonical L3 scenario;
- this Experience Slice Register records only selected bounded delivery/review increments where deeper interaction evidence is useful;
- workspace-family coverage remains in the Workspace Inventory and Screen/Journey Traceability Register.

A workflow realization may be implemented through one existing experience slice, multiple slices, shared workspace behavior, or an observable non-interactive surface.

Do not create one experience slice per scenario mechanically.
