# ADR-H-GOVLINK-001 — Government Learner Account Linking Actor and Lifecycle Clarification

| Field | Value |
|---|---|
| ADR ID | ADR-H-GOVLINK-001 |
| Change Type | L2 actor, dependency and business-lifecycle clarification |
| Target Locked Document | L2-H01 — APAAR & DigiLocker Workflows |
| Affected Downstream Scenario | L3-H-001 — Verified Student to Government Learner Account Link |
| Status | Approved — Incorporated |
| Decision Effect | Effective through the locked L2-H01 amendment dated 2026-08-26 |
| Domain | Section H — Institutional Services & Intelligence / APAAR & DigiLocker |
| Date | 2026-08-26 |

---

## 1. Need

Locked L2-H01 explicitly permits Student-initiated and Institution-initiated APAAR linking, but its command authorization matrix assigns the link-request commands only to Student and its canonical crosswalk records Actors as absent. The document also contains stale section-letter descriptions in its dependency matrix and defines link events without a concise business lifecycle for learner-account linking.

L3-H-001 therefore cannot name the institutional initiating actor or exact link lifecycle without local invention.

## 2. Architecture Constraints

The resolution shall:

- preserve H01 ownership of government-integration linking outcomes, H01 consent, connector governance, and H01 link events;
- preserve Platform Foundation ownership of Person/Identity, authorization, event and audit capabilities;
- preserve Institution & Structure ownership of Institution context and Student Lifecycle ownership of Student;
- reuse existing H01 business roles rather than inventing a new institutional title;
- preserve all six existing APAAR/DigiLocker link event names and H01 as publisher;
- avoid creating a duplicate learner-link aggregate unless required;
- keep L2 business semantics implementation independent.

## 3. Decision

### 3.1 Canonical Business Actors for Learner Linking

`Student` and `Institution Administrator` are the canonical initiating business actors for APAAR/DigiLocker learner-account linking.

- **Student** may initiate linking for their own governed learner context and performs the H01 consent actions already assigned to Student.
- **Institution Administrator** may initiate institution-authorized learner-account linking where institutional and government policy permits it.

Institution-initiated linking does not transfer Student, Person/Identity, or Institution ownership to H01 and does not bypass required consent. If policy requires Student consent, the consent must be valid regardless of who initiates the link request.

`Integration Administrator` and `Platform Administrator` retain connector-governance responsibilities only; they are not learner-link decision authorities by virtue of those roles.

### 3.2 Link Request Authorization Clarification

The business intent represented by `RequestAPAARLink` and `RequestDigiLockerLink` may originate from either:

- Student; or
- Institution Administrator, where institution initiation is permitted by governing policy.

Completion/failure remains an H01 integration outcome after authoritative government acknowledgement or rejection. This ADR defines business responsibility only; it does not define technical command handling.

### 3.3 Learner Account Link Business Lifecycle

Learner-account linking remains a specialized business path within the existing H01 integration/Publication boundary; no new aggregate is introduced.

For each platform-specific link request, the business lifecycle is:

```text
Requested → Linked
Requested → Failed
```

A `Failed` outcome is terminal for that link request. A later authorized attempt constitutes a new link request. Consent and Connector retain their separate existing lifecycles and are not merged into the link lifecycle.

The canonical events remain:

- `APAARLinkRequested`
- `APAARLinked`
- `APAARLinkFailed`
- `DigiLockerLinkRequested`
- `DigiLockerLinked`
- `DigiLockerLinkFailed`

H01 remains the canonical publisher of all six events.

### 3.4 Dependency Label Correction

For current repository terminology, H01 uses the following business dependencies where applicable:

| Section | Canonical contribution to H01 |
|---|---|
| A — Platform Foundation | Canonical Person/Identity references and governed authorization, event and audit capabilities. |
| B — Institution & Structure | Institution and institutional-structure context. |
| C — Student Lifecycle | Canonical Student / learner context. |
| D — Academic Delivery | Academic-delivery context where a specific government integration requires it. |
| I — Examination | Approved examination results and official examination/qualification evidence where the applicable H01 workflow consumes them. |

Section H may collaborate internally with Notification, Dashboard and other Institutional Services without changing source ownership. No universal direct dependency on current Section F — Finance is established by this clarification.

### 3.5 L2 Implementation-Independence Clarification

Where H01 v1.0 contains technical examples such as SDKs, APIs, protocols, queues, background workers, persistence details, or other implementation mechanisms, those references are non-normative for L2 business architecture. They do not create business actors, owners, lifecycle states, or contracts beyond the business semantics explicitly established by canonical L1/L2 decisions and this amendment.

Downstream L3 shall consume the business ownership, actors, lifecycle, events, rules and hand-offs, not those technical examples.

## 4. Impact Analysis

### L0 / L1

No new L1 change is required. The existing locked L1 Person-scope amendment dated 2026-08-26 remains authoritative for Person identity scope.

### Dependency Map

No change required. The corrected H01 labels align with the current repository dependency map.

### L2

A locked L2-H01 amendment incorporates the actor mapping, link lifecycle, dependency labels and L2 purity clarification.

### L3

L3-H-001 can name Institution Administrator for institution-initiated linking, state the exact business link lifecycle, and remove stale dependency/actor issues without inventing an aggregate or event.

## 5. Rejected Alternatives

1. Invent a new `Government Linking Officer` role — rejected because `Institution Administrator` already exists in H01 and holds institution-authorized government-publication responsibilities.
2. Let Institution Administrator bypass consent — rejected because H01 requires consent where policy requires it.
3. Create a new Learner Link aggregate — rejected because existing H01 events and integration ownership can express the business path without duplicate ownership.
4. Rename the existing six link events — rejected because they are already canonical H01 facts.
5. Preserve stale section-letter labels — rejected because higher-authority repository terminology has changed.

## 6. Acceptance Criteria

1. Institution-initiated linking has a canonical business actor.
2. Student initiation and consent responsibilities remain intact.
3. The link lifecycle is explicit without a new aggregate.
4. All six existing link events and their H01 publisher are preserved.
5. Dependency labels align with current repository domains.
6. Person, Institution and Student ownership remain outside H01.
7. L3-H-001 can be expressed with no upstream actor/dependency/lifecycle ambiguity.
8. L2 technical illustrations do not leak into L3 business architecture.

## 7. Decision Status

**Approved and incorporated on 2026-08-26 under the explicit repository-change authorization recorded in the architecture working session.**
