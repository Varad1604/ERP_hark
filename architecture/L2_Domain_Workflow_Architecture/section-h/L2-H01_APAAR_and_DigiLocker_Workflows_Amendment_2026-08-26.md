# L2-H01 — APAAR & DigiLocker Workflows — Locked Amendment

| Attribute | Value |
|---|---|
| Target | L2-H01 — APAAR & DigiLocker Workflows v1.0 |
| Amendment Version | 1.1.0 |
| Status | Locked |
| Effective Date | 2026-08-26 |
| Approved Through | ADR-H-GOVLINK-001 |
| Authority | Explicit repository-change authorization recorded 2026-08-26 |

This amendment is part of the canonical L2-H01 architecture. Where this amendment conflicts with L2-H01 v1.0, this amendment is authoritative. Unaffected L2-H01 content remains unchanged.

## 1. Canonical Business Actors for Learner Account Linking

The former Actors gap is closed for the learner-linking workflow as follows.

| Business actor | Responsibility |
|---|---|
| Student | May initiate APAAR/DigiLocker linking for their own governed learner context and performs the H01 consent actions assigned to Student. |
| Institution Administrator | May initiate institution-authorized learner-account linking where governing policy permits institution initiation. |
| Integration Administrator | Governs connector registration/configuration; does not own Student linking decisions. |
| Platform Administrator | Governs connector operational availability; does not own Student linking decisions. |

Institution initiation does not bypass consent. Where policy requires Student consent, valid consent is required regardless of the initiating actor.

## 2. Institution-Initiated Link Request Clarification

The existing H01 trigger allowing institution-initiated APAAR linking is authoritative. Accordingly, the business intent represented by `RequestAPAARLink` and `RequestDigiLockerLink` may originate from either:

- Student; or
- Institution Administrator, where allowed by institutional/government policy.

The former command-authorization wording that lists only Student for these two request intents is superseded to this extent.

This amendment defines business authorization responsibility only. It does not define technical commands, APIs, handlers, or security implementation.

## 3. Learner Account Link Business Lifecycle

Learner-account linking is a specialized business path within the existing H01 integration/Publication boundary. No separate learner-link aggregate is introduced.

Each platform-specific link request follows:

```text
Requested → Linked
Requested → Failed
```

Rules:

1. `Requested` begins when an authorized Student or Institution Administrator initiates the platform-specific link request.
2. `Linked` is reached only after the government platform has established/acknowledged the link.
3. `Failed` records a conclusive unsuccessful or rejected link outcome for that request.
4. `Failed` is terminal for that request; a later attempt is a new governed request.
5. Consent remains governed by its existing `Pending`, `Granted`, `Withdrawn`, `Expired` lifecycle.
6. Connector lifecycle remains separate from learner-link outcome.

Canonical link events remain exactly:

- `APAARLinkRequested`
- `APAARLinked`
- `APAARLinkFailed`
- `DigiLockerLinkRequested`
- `DigiLockerLinked`
- `DigiLockerLinkFailed`

H01 is the canonical publisher of all six events.

## 4. Canonical Dependency Labels

For current repository terminology, the H01 dependency matrix shall be read as follows where the applicable workflow requires the information:

| Section | Relationship to H01 |
|---|---|
| A — Platform Foundation | Supplies canonical Person/Identity references and governed authorization, event and audit capabilities. |
| B — Institution & Structure | Supplies authoritative Institution and institutional-structure context. |
| C — Student Lifecycle | Supplies the canonical Student / learner context. |
| D — Academic Delivery | Supplies academic-delivery context where required by a specific government-integration workflow. |
| I — Examination | Supplies approved results and official examination/qualification evidence where required by a specific publication workflow. |

Any older H01 section-letter description that labels B as Student, C as Academic Management, or F as Examination is superseded by this current repository mapping. No universal direct H01 dependency on current Section F — Finance is established by this correction.

## 5. Ownership Preservation

- A retains Person/Identity and Platform Foundation ownership.
- B retains Institution & Structure ownership.
- C retains Student Lifecycle ownership.
- D and I retain their academic/examination facts.
- H01 owns only its government-integration link outcome, H01 consent, connector governance, acknowledgements/synchronization where defined, and H01 events.

Participation and event consumption do not transfer ownership.

## 6. L2 Business-Architecture Purity

References in H01 v1.0 to SDKs, APIs, protocols, queues, background workers, persistence mechanisms, or similar technical constructs are non-normative implementation illustrations for purposes of L2 business architecture. They shall not be interpreted as canonical L2 business actors, ownership, lifecycle, or implementation requirements.

Canonical downstream L3 behaviour shall use business actors, responsibilities, rules, states, events, contracts and hand-offs only.

## 7. Downstream Effect

L3-H-001 shall consume this amendment and ADR-H-GOVLINK-001. Its former institutional-actor, dependency-label and learner-link-lifecycle issues are resolved.

## 8. Locked Decisions

The actor mapping, institution-initiation rule, learner-link lifecycle, existing event vocabulary/publisher, dependency mapping, ownership boundaries and L2-purity clarification in §§1–7 are Locked L2 decisions under ADR-H-GOVLINK-001.
