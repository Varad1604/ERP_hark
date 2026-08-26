# L2-E02 — Position & Assignment Workflows — Locked Amendment

| Attribute | Value |
|---|---|
| Target | L2-E02 — Position & Assignment Workflows v1.0 |
| Amendment Version | 1.1.0 |
| Status | Locked |
| Effective Date | 2026-08-26 |
| Approved Through | ADR-PLAT-AUTH-001 |
| Authority | Lead Architect approval recorded 2026-08-26 |

This amendment is part of the canonical L2-E02 architecture. Where it conflicts with L2-E02 v1.0 on the boundary between organizational responsibility and authorization-layer Role Assignment, this amendment is authoritative. All unaffected L2-E02 content remains unchanged.

## 1. Canonical Ownership Preservation

L2-E02 remains the canonical owner of:

- Position Assignment;
- Responsibility Assignment;
- reporting relationships derived from those assignments; and
- organizational authority established through those assignments.

L2-A04 remains the canonical owner of authorization-layer Role, Role Assignment, Permission, Authorization Policy and Authorization Decision.

These ownership boundaries are distinct and shall not be merged.

## 2. Organizational Responsibility vs Authorization Role Assignment

An E02 Responsibility Assignment establishes an organizational responsibility or organizational-authority fact. It does **not** directly grant an A04 Permission and does not create an A04 Role Assignment.

An A04 Role Assignment establishes authorization-layer responsibility for later Role Resolution, Permission Resolution and policy evaluation. It does **not** create, replace or prove E02 organizational authority.

Accordingly:

- organizational authority shall not be derived directly from an A04 Role or Role Assignment;
- authorization access shall not be inferred merely from the existence of an E02 Responsibility Assignment; and
- each aggregate retains its own lifecycle and audit history under its canonical owner.

## 3. Governed E02-to-A04 Hand-off

When an E02-owned organizational responsibility requires authorization-layer access, the originating E02/business process may initiate the A04 **Request Role Assignment** business contract.

The request shall provide or reference, as applicable:

- the canonical A02 Person who is the subject;
- the E02 Responsibility Assignment or other source business evidence establishing the organizational basis;
- the requested authorization-layer responsibility or Role;
- the requested authorization scope;
- the applicable effective period; and
- originating business context sufficient for A04 policy and audit evaluation.

A04 then independently governs:

- assignment scope;
- least privilege;
- Separation of Duties;
- required approval;
- Role Assignment lifecycle; and
- authorization-layer events.

This is a business hand-off only. No API, message schema, storage model or technical interface is defined.

## 4. Dependency Direction

This hand-off does not make foundational A04 Authorization & Policy depend on L2-E02.

The dependency direction remains:

- E02 or another consuming business process may consume A04 authorization capability;
- A04 may reference source business evidence presented with a request without becoming dependent on the source domain for its own existence or lifecycle; and
- participation in a Role Assignment request does not transfer ownership of organizational or authorization facts.

## 5. Event Boundary

This amendment does not invent a new E02 business-event name.

E02 continues to publish only events already canonically established by its own architecture. The cross-domain initiation of authorization is defined here as the governed **Request Role Assignment** business contract into A04.

Once A04 accepts the request into its lifecycle, A04 Role Assignment exclusively publishes the canonical Role Assignment events defined by the L2-A04 locked amendment approved through ADR-PLAT-AUTH-001.

## 6. L3-A-010 Boundary Clarification

L3-A-010 begins when an authorization-layer Role Assignment requirement for an eligible Person has reached the A04 scenario boundary.

The source establishment of an organizational responsibility, including any E02 Responsibility Assignment, remains upstream and outside the A010 scenario boundary. Therefore this amendment does not require changing A010's frozen:

- Primary Owner; or
- Participating Domains value `A`.

## 7. Locked Decisions

The following are Locked under ADR-PLAT-AUTH-001:

1. E02 organizational responsibility/authority and A04 authorization Role Assignment are separate business concepts.
2. E02 does not directly grant A04 Permissions.
3. A04 does not create or replace E02 organizational authority.
4. An E02 responsibility may serve as source evidence for a governed A04 Request Role Assignment.
5. The hand-off does not reverse foundational dependency direction or transfer ownership.
6. No new E02 event is created by this amendment solely to support the hand-off.