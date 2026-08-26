# ADR-G-INF-REG-001 — Physical Campus Registration Event and Initial Lifecycle Clarification

| Field | Value |
|---|---|
| ADR ID | ADR-G-INF-REG-001 |
| Change Type | L2 event and lifecycle contradiction resolution |
| Target Locked Document | L2-G10 — Infrastructure Management Workflows |
| Blocked Downstream Scenario | L3-G-278 — Institutional Infrastructure Registration to Physical Campus Establishment |
| Status | Approved — Incorporated |
| Decision Effect | Effective through the locked L2-G10 amendment dated 2026-08-26 |
| Domain | Section G — Campus Operations / Infrastructure Management |
| Date | 2026-08-26 |

---

## 1. Need

Locked L2-G10 contains two internal contradictions affecting Physical Campus registration.

First, the dedicated Registration Events catalogue names `CampusRegistered`, while WF-INF-001, the workflow summary, and structural-event examples use `CampusCreated` for the same completed Physical Campus registration fact.

Second, registration sections state that newly registered infrastructure starts in `Draft`, while the canonical state model in §11A begins at `Proposed` and defines `Proposed → Planned → Approved → Under Construction → Operational`. WF-INF-001 further states the initial lifecycle as `Proposed or Planned`.

L3-G-278 cannot choose an event or initial state locally without changing locked L2 architecture.

## 2. Architecture Constraints

The resolution shall:

- preserve G10 ownership of the Physical Campus and physical infrastructure lifecycle;
- preserve B03 ownership of InstitutionalCampus identity and Institution-to-Campus governance;
- preserve the canonical §11A infrastructure lifecycle and its operational-availability rules;
- establish one canonical event for one completed registration fact;
- avoid duplicate event publication and avoid introducing a second lifecycle;
- remain implementation independent.

## 3. Decision

### 3.1 Canonical Registration Event

`CampusRegistered` is the canonical business event for successful registration of a G10 Physical Campus.

**Publisher:** Infrastructure Management / G10.  
**Meaning:** A canonical Physical Campus has been registered with immutable identity and required physical registration information under G10 ownership.

Where `CampusCreated` in L2-G10 refers to this same Physical Campus registration completion, that wording is superseded. `CampusCreated` shall not be published as a duplicate alias for the same registration fact.

This decision does not rename `BuildingCreated`, `FloorCreated`, `SpaceCreated`, or unrelated infrastructure events.

### 3.2 Canonical Initial Lifecycle State

A newly registered Physical Campus enters the canonical lifecycle in `Proposed`.

The applicable Physical Campus lifecycle is:

```text
Proposed → Planned → Approved → Under Construction → Operational
```

with the existing §11A governed operational branches and terminal progression retained.

`Draft` is not a canonical Physical Campus lifecycle state after this amendment. `Planned` is not a permitted initial state for a newly registered Physical Campus because the canonical transition model begins at `Proposed` and does not authorize skipping that state.

### 3.3 Registration Does Not Mean Operational Availability

Successful registration establishes canonical physical identity only. A registered Physical Campus in `Proposed` is not operationally available. It becomes available only when it reaches the existing canonical `Operational` state.

### 3.4 Ownership Boundary

The Physical Campus may reference the B03-owned InstitutionalCampus / institutional context. Registration does not transfer InstitutionalCampus governance to G10 and does not transfer Physical Campus ownership to B03.

## 4. Impact Analysis

### L0 / L1

No change required. Existing Section B / Section G ownership boundaries remain unchanged.

### Dependency Map

No change required. The dependency map already distinguishes Institutional Campus governance from G10 physical-site ownership.

### L2

A locked L2-G10 amendment supersedes conflicting `CampusCreated`, `Draft`, and `Proposed or Planned` wording only where it describes Physical Campus registration and its canonical lifecycle.

### L3

L3-G-278 can now complete registration using `CampusRegistered` and the initial `Proposed` state without local interpretation.

## 5. Rejected Alternatives

1. Keep both `CampusRegistered` and `CampusCreated` — rejected because one business fact would have two canonical events.
2. Choose `CampusCreated` — rejected because the dedicated registration-event catalogue defines the semantically precise `CampusRegistered` event for registration.
3. Retain `Draft` as a second lifecycle — rejected because §11A defines the canonical infrastructure lifecycle and its transition rules.
4. Allow `Proposed or Planned` as the initial state — rejected because it makes the start state non-deterministic and can skip the canonical `Proposed → Planned` transition.
5. Move Physical Campus ownership to B03 — rejected because B03 owns institutional campus governance while G10 owns the physical representation.

## 6. Acceptance Criteria

1. Physical Campus registration has one event: `CampusRegistered`.
2. The event has one publisher: G10.
3. A newly registered Physical Campus starts in `Proposed`.
4. Registration does not imply operational availability.
5. B03/G10 ownership separation remains intact.
6. L3-G-278 is no longer blocked by event or state ambiguity.
7. No implementation architecture is introduced.

## 7. Decision Status

**Approved and incorporated on 2026-08-26 under the explicit repository-change authorization recorded in the architecture working session.**
