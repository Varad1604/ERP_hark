# Student Charge Dispute to Governed Resolution

**Scenario ID:** L3-F-004  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Student Charge & Fee Management  
**Participating Domains:** C — Student Lifecycle; F — Finance  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** L1 Business Domain Architecture; DEPENDENCY_MAP.md; L2-F00 Finance Domain Model & Foundation; L2-F01 Student Charge & Fee Management; authoritative Student Lifecycle source architecture where the dispute originates  
**Referenced By:** Payment, Refund, Financial Adjustment, Accounts Receivable and Accounting capabilities where a governed dispute outcome creates a downstream financial consequence

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | L3-F-004 |
| Canonical Title | Student Charge Dispute to Governed Resolution |
| Primary Owner | Student Charge & Fee Management |
| Participating Domains | C — Student Lifecycle; F — Finance |
| Scenario Type | Critical |
| Priority | P1 |
| Status | Draft |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

Provide a complete institutional path for a disputed Student Charge to reach a governed financial resolution while preserving Student Charge ownership, financial history, and the separation between the originating student/business context and Finance's canonical financial obligation.

The scenario covers the business resolution of the disputed charge and stops once the authoritative resolution has been reached and the resulting financial consequence has been handed to its canonical Finance owner where required.

## 3. Business Outcome

The disputed Student Charge reaches one governed resolution that is authoritative for the financial position.

The resolution may confirm the existing charge, or may result in a governed financial correction through the applicable Financial Adjustment or other canonical Finance capability. The original Student Charge remains the canonical financial obligation and its history remains traceable.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario begins when a Student Charge is formally identified as disputed through an authoritative business context.

### 4.2 End Boundary

The scenario ends when the dispute has reached its governed financial resolution and the resulting outcome has been applied or handed to the canonical Finance capability responsible for that consequence.

### 4.3 In Scope

- Identification of the disputed Student Charge.
- Establishment of the dispute context.
- Review of the disputed financial position.
- Determination of the governed resolution.
- Confirmation of the original charge where it remains correct.
- Referral to the canonical Financial Adjustment capability where a financial correction is required.
- Referral to Refund Management where the governed resolution produces a refund consequence.
- Preservation of financial and dispute lineage.
- Communication of the completed resolution through the canonical business collaboration mechanism defined upstream.

### 4.4 Out of Scope

- Creating or redefining the Student Charge.
- Rewriting historical published Student Charge information.
- Payment execution.
- Refund execution.
- Financial Adjustment execution beyond the hand-off to its owning capability.
- Accounting posting.
- Technical case management, APIs, persistence, queues, retries, or implementation mechanics.

## 5. Ownership and Participating Domains

| Domain | Scenario Responsibility | Canonical Ownership Preserved |
|---|---|---|
| C — Student Lifecycle | Provides the authoritative student context where the dispute concerns a student-owned business fact. | Student lifecycle concepts remain C-owned. |
| F — Student Charge & Fee Management | Owns the Student Charge and the financial interpretation of the disputed obligation. | Student Charge and its lifecycle remain F-owned. |
| F — Financial Adjustment Management | Owns any approved financial correction that is required after dispute resolution. | Financial Adjustment remains F04-owned. |
| F — Refund Management | Owns a refund consequence where the governed resolution requires return of monetary value. | Refund remains F03-owned. |
| F — Accounts Receivable | May consume the resulting receivable consequence where the dispute changes the receivable position. | Receivable remains F07-owned. |

Participation does not transfer ownership. L1 establishes that Finance owns monetary operations and that domains communicate through contracts and events rather than direct cross-domain writes.

## 6. Business Actors, Roles and Responsibilities

L2-F01 does not define an authoritative actor catalogue; its canonical-section crosswalk explicitly records Actors as absent. Therefore no Finance dispute role, approver, investigator, or adjudicator is invented here.

The originating business domain remains responsible for any source-domain actor or business authority that establishes the dispute context. Any Finance review or decision authority must come from the applicable governed Finance/enterprise approval architecture.

## 7. Preconditions and Entry Criteria

- A canonical Student Charge exists.
- The charge is sufficiently identifiable to establish the disputed financial position.
- An authoritative dispute context exists.
- The dispute concerns a business or financial fact within the scenario boundary.
- The original Student Charge remains available as the canonical historical financial obligation.
- Any downstream corrective outcome can be represented by an existing canonical Finance capability.

## 8. Trigger

An authoritative business context identifies a Student Charge as disputed and requires a governed financial resolution.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Domain | Business Result |
|---|---|---|---|
| 1 | The dispute is established against the canonical Student Charge. | C / F as applicable | A specific financial obligation is under governed dispute. |
| 2 | The affected Student Charge and its relevant financial context are identified. | F — Student Charge | The dispute is tied to the authoritative financial obligation. |
| 3 | The disputed basis is reviewed against the authoritative charge context, applicable classification, policy, calculation and historical lineage. | F — Student Charge | The financial position relevant to the dispute is established. |
| 4 | The dispute is evaluated to determine whether the existing Student Charge remains correct or requires a governed financial consequence. | F | A resolution path is determined. |
| 5 | Where the charge is confirmed, the existing Student Charge remains authoritative and the dispute is resolved without altering the charge. | F | The original financial obligation remains valid. |
| 6 | Where a financial correction is required, the correction is handed to Financial Adjustment Management under its canonical ownership. | F — Financial Adjustment | A governed correction path exists without rewriting the historical charge. |
| 7 | Where the resolution requires monetary return, the consequence is handed to Refund Management under its canonical ownership. | F — Refund Management | A governed refund path exists. |
| 8 | Where the resolution changes receivable position, the resulting financial fact is made available to Accounts Receivable. | F — Accounts Receivable | Receivable representation can reflect the authoritative financial outcome. |
| 9 | The dispute resolution and resulting financial consequence remain traceable to the original Student Charge. | F | The institutional financial position has a reconstructable resolution history. |

The flow stops when the dispute has a governed resolution. It does not expand into payment, refund, adjustment, or accounting execution.

## 10. Alternate Business Flows

### A1 — Charge Confirmed

The review establishes that the Student Charge remains correct. No change to the canonical charge is made.

### A2 — Financial Adjustment Required

The dispute establishes a valid need to modify the existing financial position. The Financial Adjustment capability becomes responsible for the correction.

L2-F04 defines Financial Adjustment as the canonical aggregate for an approved modification to an existing financial position and explicitly separates adjustments from Student Charges, Payments and Refunds.

### A3 — Refund Consequence

The dispute resolution establishes that monetary value must be returned. Refund Management becomes responsible for the refund outcome.

### A4 — Receivable Consequence

The dispute resolution changes the outstanding receivable position. Accounts Receivable consumes the authoritative financial consequence while preserving Student Charge ownership.

### A5 — Insufficient Resolution Basis

The available business evidence does not support a governed resolution. The dispute remains unresolved until the authoritative missing business context is provided.

## 11. Failure, Interruption and Business Recovery

| Business Situation | Business Resolution |
|---|---|
| Student Charge cannot be identified | Do not resolve the dispute against an inferred obligation; establish the correct canonical charge first. |
| Dispute basis is insufficient | Keep the dispute unresolved and obtain the authoritative business context required for a decision. |
| Conflicting business facts exist | Preserve the unambiguous facts and escalate the conflict to the owning authority; do not invent a compromise. |
| Requested resolution would directly rewrite historical published charge information | Reject that path and use the applicable governed financial correction capability. |
| Financial consequence crosses into another Finance capability | Hand the consequence to the canonical owner rather than creating a local duplicate process. |

No technical retry, rollback, queue, transport, or infrastructure recovery is defined at L3.

## 12. Business Rules and Decision Constraints

The scenario applies the following authoritative constraints:

- Student Charge is the canonical payable financial obligation.
- Student Charge is exclusively owned by Student Charge & Fee Management.
- Published Student Charges remain historically immutable.
- Financial corrections are represented through the governed Financial Adjustment capability rather than rewriting historical obligations.
- Payment, Refund, Accounting and Accounts Receivable remain separate Finance responsibilities.
- External domains do not directly modify Student Charges.
- Complete financial lineage must remain reconstructable.

L2-F01 establishes the canonical Student Charge ownership, immutable published-charge principle, and separation of obligation from settlement. L2-F04 establishes Financial Adjustment as the canonical modification of an existing financial position.

No dispute-specific business rule, authority, state, or event name is invented.

## 13. State and Lifecycle Impact

The Student Charge remains under the canonical lifecycle and state architecture owned by L2-F01.

The scenario does not introduce a new Student Charge lifecycle or state.

Where the dispute results in a downstream adjustment or refund, the affected downstream aggregate follows its own canonical lifecycle and state architecture.

A dispute resolution therefore changes the governed financial position through the appropriate owning capability rather than creating a competing Student Charge lifecycle.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Business Events

The accessible locked L2-F01 event architecture defines canonical Student Charge lifecycle events such as:

- `ChargeIdentificationInitiated`
- `ChargeGenerationCompleted`
- `ChargeValidationCompleted`
- `ChargePublicationCompleted`
- `ChargeActivationCompleted`
- `ChargeClosureCompleted`
- `ChargeArchivalCompleted`

It does **not** define a dedicated Student Charge dispute event in the accessible canonical event list. The scenario therefore does not invent a dispute event name or publisher.

Financial Adjustment Management and Accounts Receivable have their own canonical event vocabularies. Those events remain published by their owning domains.

### 14.2 Synchronous Business Contracts / Hand-offs

| From | To | Business Hand-off | Purpose |
|---|---|---|---|
| C / originating business context | F — Student Charge | Authoritative dispute context | Establishes the business basis for review. |
| F — Student Charge | Financial Adjustment | Dispute resolution requiring financial correction | Transfers correction responsibility without transferring Student Charge ownership. |
| F — Student Charge | Refund Management | Refund consequence | Transfers refund responsibility to the canonical owner. |
| F — Student Charge | Accounts Receivable | Authoritative changed financial position | Allows receivable representation to reflect the resolved outcome. |

Events remain distinct from these business hand-offs.

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Business Authority

No dispute-specific business authority is defined in the accessible L2-F01 source. The scenario therefore requires the applicable upstream/enterprise authority to govern the review and decision rather than inventing a role.

### 15.2 Separation of Duties

No dispute-specific separation-of-duty requirement is explicitly defined in the accessible L2-F01 or L2-F04 material. No new separation rule is introduced.

### 15.3 Business Audit and Evidence Obligations

The dispute resolution must remain traceable to:

- the canonical Student Charge;
- the originating dispute context;
- the governed resolution;
- any resulting Financial Adjustment, Refund, or Receivable consequence.

L2-F01 requires complete financial lineage and auditability across charge generation, validation, publication, lifecycle and state changes.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- The disputed Student Charge is identified.
- The dispute reaches a governed resolution or is formally left unresolved because authoritative business context is insufficient.
- The original Student Charge remains the canonical financial obligation.
- Any required financial correction is handed to Financial Adjustment Management.
- Any required refund is handed to Refund Management.
- Any resulting receivable consequence is available to Accounts Receivable.
- The resolution remains traceable to the originating Student Charge.

### 16.2 Postconditions

- The institution has an authoritative financial resolution for the dispute, or the dispute remains explicitly unresolved pending authoritative information.
- No duplicate financial obligation has been created.
- Historical Student Charge information remains intact.
- Downstream financial consequences remain owned by their canonical domains.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| L2-F01's canonical-section crosswalk records Actors as not present. | A dispute-specific actor/authority cannot be stated authoritatively in L3. | Confirm the applicable business actor/authority in the owning Finance/enterprise governance architecture. |
| L2-F01's accessible event architecture does not define a dedicated dispute event. | L3 cannot name a canonical dispute event or publisher without inventing architecture. | Confirm whether a canonical dispute event exists in an authoritative upstream source; if not, govern it before L3 approval. |
| L2-F01 does not expose a dedicated dispute lifecycle/state model. | The scenario cannot safely define a dispute-specific state transition. | Confirm the authoritative dispute lifecycle/state ownership before Architecture Approval. |

These gaps are material to actor authority and event/state integrity; they must not be silently resolved in L3.

### 17.2 Assumptions

No assumptions override locked architecture.

### 17.3 Deferred Questions

No implementation-level deferred questions are required.
