# Damaged Library Resource During Stock Verification to Administrative Review

**Scenario ID:** L3-G-148  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Library Management  
**Participating Domains:** G, F, E, H  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** Locked L1 Business Domain Architecture; L2 Domain Workflow Architecture  
**Referenced By:** None yet  

---

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | L3-G-148 |
| Canonical Title | Damaged Library Resource During Stock Verification to Administrative Review |
| Primary Owner | Library Management |
| Participating Domains | G, F, E, H |
| Scenario Type | Cross-domain |
| Priority | P1 |
| Status | Locked |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

This scenario governs the institutional outcome in which a damaged library resource is identified during stock verification, its condition is assessed through the applicable business review, and an administrative outcome is established for subsequent handling.

The locked L2 explicitly states that verification may identify damaged resources and that operational handling remains within Library Management. It also identifies severe damage as one possible basis for a write-off recommendation, with final approval following institutional policy.

This scenario therefore does not invent a local damage lifecycle or automatically convert damage into write-off, replacement, fine, or disciplinary action.

## 3. Business Outcome

A library resource identified as damaged during stock verification has undergone administrative review and the resulting business determination is recorded.

The determination may establish that:

- the resource remains operationally usable subject to the authoritative condition;
- further administrative handling is required; or
- severe damage supports progression toward a write-off recommendation.

The exact downstream decision must remain governed by authoritative institutional policy.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

A damaged resource is identified during an authorised stock-verification process.

### 4.2 End Boundary

The damage finding has been reviewed administratively and the applicable business outcome is recorded.

### 4.3 In Scope

- Damage identification in verification.
- Physical condition observation.
- Difference analysis context.
- Administrative review.
- Cross-domain financial/personnel/notification participation where required.
- Preservation of review history.
- Progression to an authorised downstream action where supported.

### 4.4 Out of Scope

- Inventing damage severity thresholds.
- Inventing replacement charges.
- Automatic fine calculation.
- Final write-off approval.
- Procurement.
- Accounting/payment.
- Technical inspection mechanisms.
- Physical security implementation.
- UI/infrastructure.

## 5. Ownership and Participating Domains

### Library Management — G
Owns stock-verification findings, operational damage handling and administrative review.

### Finance — F
Owns accounting/payment processing and supplies applicable financial information where required.

### People & Workforce — E
Provides authoritative personnel context where required.

### Communication — H
Handles notification delivery where required.

Cross-domain participation does not transfer ownership from G.

## 6. Business Actors, Roles and Responsibilities

- **Library Verification Personnel:** identify and record the damaged-resource finding.
- **Library Administrative Authority:** reviews the damage outcome and establishes the applicable administrative decision.
- **E:** provides authoritative personnel context where required.
- **F:** provides financial information where applicable.
- **H:** delivers governed communications where applicable.

No unsupported damage assessor, disciplinary officer, or finance approver is invented.

## 7. Preconditions and Entry Criteria

1. Stock verification is authorised.
2. The resource is within the verification scope.
3. The resource is identified as damaged during physical verification.
4. The damage finding can be associated with the authoritative resource record.
5. Administrative review authority is available.
6. Required cross-domain context is available where applicable.

## 8. Trigger

Identification of a damaged resource during physical verification.

The L2 explicitly lists damaged resources as a possible verification result.

No new damage event is invented because the retrieved L2 event table defines verification events but does not define a dedicated `DamagedResourceIdentified` event.

## 9. Main Business Flow

### Phase A — Establish Damage Finding

1. Identify the verification cycle.
2. Identify the affected library resource.
3. Record the physical-condition/damage observation within the verification context.
4. Establish that the resource has been identified as damaged.
5. Preserve the verification finding.

### Phase B — Establish Administrative Review

6. Bring the damage finding into Library Management administrative review.
7. Consider the authoritative resource condition.
8. Consider relevant verification evidence.
9. Determine whether additional administrative handling is required.
10. Where personnel context is relevant, obtain authoritative information from E.
11. Where financial implications are relevant, obtain applicable information from F.

### Phase C — Establish Administrative Outcome

12. Establish the administrative review outcome.
13. Preserve the decision and supporting business evidence.
14. Where the outcome requires a downstream action, establish that action as the next business boundary.
15. Where notification is required, provide the governed business outcome to H.

### Phase D — Complete

16. End the scenario with the damage finding administratively reviewed and the applicable next action established.

The scenario does not automatically execute procurement, payment, final write-off approval, or disciplinary action.

## 10. Alternate Business Flows

### 10.1 Minor/Operational Damage

Where the authoritative review determines that the resource remains usable, the resource remains subject to the applicable operational condition.

No local `Minor Damage` state is invented.

### 10.2 Damage Requires Further Handling

The administrative review establishes the appropriate downstream handling under institutional policy.

### 10.3 Severe Damage

The L2 identifies severe damage as a possible basis for a write-off recommendation. The scenario may therefore establish progression toward write-off review, but it does not invent or grant final write-off approval.

### 10.4 Financial Context Required

F provides applicable financial information. Accounting/payment remains Finance-owned.

### 10.5 Personnel Context Required

E provides authoritative personnel context.

### 10.6 Notification Required

H handles delivery of the governed communication.

### 10.7 Damage Finding Is Disputed or Insufficient

Administrative review remains unresolved until the authoritative business evidence is sufficient.

## 11. Failure, Interruption and Business Recovery

### 11.1 Resource Identity Cannot Be Established

No damage outcome is attached to an invented resource.

### 11.2 Damage Evidence Is Incomplete

Administrative review remains unresolved until sufficient authoritative evidence exists.

### 11.3 Financial Information Pending

G does not invent a replacement charge, fine, or accounting amount. F remains authoritative for financial processing.

### 11.4 Personnel Context Pending

E remains authoritative for personnel information.

### 11.5 Administrative Outcome Pending

The resource remains under the applicable review condition; no unsupported final action is asserted.

### 11.6 Recovery Boundary

Once the authoritative review is completed, the resource progresses to the applicable business outcome.

No technical retries, queues, rollback, API, database, infrastructure or deployment behaviour is specified.

## 12. Business Rules and Decision Constraints

The L2 states that verification may identify damaged resources and that operational handling remains within Library Management.

The L2's Fine Management identifies **Damaged Resource** and **Replacement Charge** as configurable fine categories, but it does not establish that every stock-verification damage finding automatically creates a fine. Fine calculation remains configuration driven and Finance owns accounting/payment.

The L2 also states that severe damage may support a write-off recommendation, with final approval governed by institutional policy.

Therefore:

1. No damage severity threshold is invented.
2. No replacement amount is invented.
3. No fine is automatically created.
4. No write-off is automatically approved.
5. No procurement outcome is invented.
6. Cross-domain information does not transfer ownership.

## 13. State and Lifecycle Impact

The authoritative stock-verification workflow is:

`Verification Scheduled → Physical Verification → Difference Analysis → Missing Resources → Verification Report → Administrative Closure`.

Damage is explicitly a verification result rather than a separate lifecycle state in the L2.

This scenario therefore does not invent:

- `Damaged`.
- `Damage Under Review`.
- `Repair Required`.
- `Write-Off Approved`.

The administrative review outcome determines the downstream business boundary.

## 14. Business Events and Cross-Domain Collaboration

The L2 defines:

- `LibraryVerificationStarted`
- `LibraryVerificationCompleted`
- `MissingResourceIdentified`

but does **not** define a dedicated damage-identification event.

Therefore no `DamagedResourceIdentified` event is invented.

### G — Library Management
Owns the verification damage finding and administrative review.

### F — Finance
Provides/consumes financial information where applicable; owns accounting/payment.

### E — People & Workforce
Provides personnel context where applicable.

### H — Communication
Handles notification delivery where required.

## 15. Business Authorization, Approval and Audit Obligations

The damage finding and administrative review must remain auditable.

Relevant evidence includes:

- Verification identity.
- Resource identity.
- Physical-condition/damage observation.
- Verification evidence.
- Administrative review outcome.
- Financial information where applicable.
- Personnel context where applicable.
- Downstream action established.
- Communication outcome where applicable.

Final write-off approval remains governed by institutional policy and is not invented here.

## 16. Completion Criteria and Postconditions

### Completion Criteria

1. Verification cycle identified.
2. Damaged resource identified.
3. Damage observation preserved.
4. Administrative review completed.
5. Applicable outcome established.
6. Relevant cross-domain context incorporated where required.
7. Review history preserved.
8. Any downstream action is established without silently executing an unrelated workflow.

### Postconditions

- Damage finding has an administrative outcome.
- The resource remains under its authoritative library condition.
- If severe damage supports write-off review, the next boundary is established but final approval is not implied.
- No automatic fine is created unless authoritative rules require it.
- No procurement or payment is created.
- G remains owner.

## 17. Open Issues, Assumptions and Deferred Questions

### Open Issues

1. The L2 does not define a dedicated damage event.
2. It does not define a detailed damage-severity decision matrix.
3. It does not define when damage automatically triggers a fine versus administrative review versus write-off review.
4. Final write-off approval authority is governed by institutional policy but is not specified here.

### Assumptions

1. Damage is a stock-verification result.
2. Administrative review remains G-owned.
3. Financial consequences remain governed by Finance/institutional policy.
4. Severe damage can support write-off review only as stated by L2.

### Deferred Questions

Confirm the authoritative institutional policy for damage severity, fine applicability, replacement obligations and write-off approval before Architecture Approval/Lock.

No local rule is invented.
