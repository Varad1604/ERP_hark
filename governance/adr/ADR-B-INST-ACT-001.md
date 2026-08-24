# ADR-B-INST-ACT-001 — Institution Activation Authority Clarification

| Field | Value |
|---|---|
| ADR ID | ADR-B-INST-ACT-001 |
| Change Type | L2 business-authority clarification |
| Target Locked Document | L2-B01 — Institution Workflows |
| Blocked Downstream Scenario | L3-B-001 — Institution Establishment to Final Activation Decision |
| Status | Approved — Incorporated |
| Decision Effect | Effective through the locked L2-B01 amendment dated 2026-08-24 |
| Domain | Section B — Institution & Structure / Institution |
| Date | 2026-08-24 |

---

## 1. Need

Locked `L2-B01 — Institution Workflows` owns Institution activation and defines the lifecycle transition through `Pending Approval` to `Active`, `Returned`, or `Rejected`. It also states that a pending activation is awaiting an administrative or board decision, but its canonical crosswalk records that an Actors section is absent and the workflow does not define one stable business authority name for the final activation decision.

This leaves `L3-B-001` unable to state the actor and decision authority without inventing a local title such as Principal, Registrar, Board, Trustee, or Administrator.

## 2. Architecture Constraints

The resolution shall:

- preserve Institution ownership of Institution identity, lifecycle, status, configuration, policy, activation, suspension, reactivation, and archival;
- preserve the existing Institution lifecycle and the existing `InstitutionActivated` publisher;
- remain valid for schools, colleges, universities, autonomous institutions, affiliated institutions, and other supported educational institutions;
- avoid hard-coding one institution-specific job title or governing body;
- keep institutional business-policy ownership in the Institution domain while preserving L2-A04 ownership of Authorization Policy and authorization enforcement;
- remain implementation independent and define no APIs, database fields, UI permissions, technical RBAC configuration, or deployment behaviour;
- introduce no new ownership transfer, lifecycle state, event publisher, or cross-domain dependency.

## 3. Decision

### 3.1 Canonical Initiating Role

**Institution Administrator** is the canonical business role responsible for establishing and configuring the Institution context and submitting an Institution activation request for governed decision.

This reuses the `Institution Administrator` terminology already present in L2-B01 mandatory administrative information.

### 3.2 Canonical Decision Authority

**Institution Activation Authority** is the canonical business authority responsible for the final Institution activation decision.

The Institution Activation Authority may:

- approve a valid activation request, allowing the Institution to reach `Active`;
- reject the activation request, producing the existing `Rejected` lifecycle outcome; or
- return the request for correction where the existing `Returned` path applies.

The authority acts only after the existing L2-B01 activation prerequisites and validation rules have been satisfied or evaluated.

### 3.3 Institution-Independent Role Mapping

`Institution Activation Authority` is an abstract business authority, not a universal job title.

An institution may govern which local office, committee, board, or other authorized actor fulfils this business authority according to its institutional governance and policy. The authorization mechanism that maps and enforces access to that authority remains owned by L2-A04 and later implementation architecture; L2-B01 owns only the business decision responsibility.

No universal Principal, Registrar, Board, Trustee, Tenant Administrator, or other fixed title is made canonical by this decision.

### 3.4 Separation of Duties

This ADR introduces no universal separation-of-duties rule between the Institution Administrator and Institution Activation Authority because the locked upstream architecture does not establish one. Institution-specific governance may impose stronger separation through its authoritative business policy and authorization model.

### 3.5 Existing Lifecycle and Events Remain Unchanged

This decision does not change the canonical Institution lifecycle:

`Draft → Configured → Pending Approval → Active / Returned / Rejected`, with the existing governed suspension, reactivation, and archival paths retained.

The canonical public activation event remains:

- `InstitutionActivated` — published by the Institution domain when the Institution becomes `Active`.

No duplicate activation event is introduced.

## 4. Structural Completion Effect on L2-B01

The associated locked amendment also closes the existing L2-B01 canonical-section gaps for:

- Actors;
- Entities;
- Contracts; and
- Locked Decisions.

Entities and contracts are consolidated only from architecture already present in L1/L2-B01. The only new canonical business concept established by this ADR is the `Institution Activation Authority` business authority described above.

## 5. Impact Analysis

### L0 / L1

No change required. Institution ownership remains unchanged.

### Dependency Map

No dependency change required. Existing Tenant-to-Institution and Institution-to-downstream relationships remain unchanged.

### L2

A locked L2-B01 amendment is required to incorporate the authority decision and structurally complete the missing canonical sections.

### L3

`L3-B-001` may replace its former actor/authority uncertainty with the canonical roles defined by this ADR and the L2-B01 amendment. No scenario ID, title, primary owner, type, priority, event name, or lifecycle ownership changes.

## 6. Rejected Alternatives

1. **Hard-code Principal or Registrar as approver** — rejected because it is not institution-independent.
2. **Hard-code Board approval** — rejected because L2-B01 uses admin/board language descriptively and does not establish Board as the universal canonical authority.
3. **Let L3 choose the approver per scenario** — rejected because L3 cannot create upstream business authority.
4. **Move activation authority to Platform Authorization** — rejected because L2-A04 owns authorization policy/enforcement, not the Institution business decision itself.

## 7. Acceptance Criteria

1. Institution activation has one canonical business decision authority name.
2. The authority is institution-independent and does not hard-code a local job title.
3. Institution ownership and the existing lifecycle remain unchanged.
4. `InstitutionActivated` retains one canonical publisher: Institution.
5. L2-A04 authorization ownership remains intact.
6. L3-B-001 can identify initiation and final-decision responsibility without local invention.
7. No implementation architecture is introduced into L2/L3.

## 8. Decision Status

**Approved and incorporated on 2026-08-24 under the explicit repository-change authorization recorded in the architecture working session.**
