# L2-G10 — Infrastructure Management Workflows — Locked Amendment

| Attribute | Value |
|---|---|
| Target | L2-G10 — Infrastructure Management Workflows v1.0 |
| Amendment Version | 1.1.0 |
| Status | Locked |
| Effective Date | 2026-08-26 |
| Approved Through | ADR-G-INF-REG-001 |
| Authority | Explicit repository-change authorization recorded 2026-08-26 |

This amendment is part of the canonical L2-G10 architecture. Where this amendment conflicts with L2-G10 v1.0, this amendment is authoritative. Unaffected L2-G10 content remains unchanged.

## 1. Physical Campus Registration Event

The canonical event for successful registration of a Physical Campus is:

- `CampusRegistered`

**Canonical publisher:** Infrastructure Management / G10.

Where WF-INF-001, the workflow event summary, structural-event examples, or other L2-G10 text use `CampusCreated` for the same completed Physical Campus registration fact, `CampusCreated` is superseded by `CampusRegistered` for that fact and shall not be published as a duplicate alias.

This amendment does not rename `BuildingCreated`, `FloorCreated`, `SpaceCreated`, or unrelated infrastructure events.

## 2. Physical Campus Initial Lifecycle

A newly registered Physical Campus enters the canonical §11A lifecycle in:

- `Proposed`

The canonical start progression is:

```text
Proposed → Planned → Approved → Under Construction → Operational
```

All existing §11A governed branches after `Operational`, including maintenance, renovation, temporary closure, decommissioning and archival, remain unchanged.

For Physical Campus registration:

- earlier `Draft` initial-state wording is superseded;
- `Initial Lifecycle = Proposed or Planned` is superseded by `Initial Lifecycle = Proposed`;
- older illustrative Campus state sequences using `Draft`, `Commissioned`, `Closed`, or other non-§11A terms do not redefine the canonical Physical Campus lifecycle.

This clarification applies to the Physical Campus scenario addressed by ADR-G-INF-REG-001. It does not replace separate lifecycles of auxiliary aggregates such as Compliance, Inspection, or Reservation where G10 defines their own states.

## 3. Registration and Operational Availability

Registration establishes canonical Physical Campus identity and registration evidence. It does not establish operational availability.

A Physical Campus in `Proposed`, `Planned`, `Approved`, or `Under Construction` is not operationally available. Existing G10 rules that only `Operational` infrastructure may be used by operational domains remain authoritative.

## 4. Ownership Boundary

- G10 retains exclusive ownership of Physical Campus identity, physical attributes and physical lifecycle.
- B03 retains InstitutionalCampus identity and Institution-to-Campus governance.
- A reference from G10 to B03 does not transfer ownership in either direction.

## 5. Downstream Effect

L3-G-278 shall consume this amendment and ADR-G-INF-REG-001. Its former event-name blocker and initial-lifecycle ambiguity are resolved.

## 6. Locked Decisions

The `CampusRegistered` event, G10 publisher, `Proposed` initial state, registration/non-operational distinction, and B03/G10 ownership boundary in §§1–5 are Locked L2 decisions under ADR-G-INF-REG-001.
