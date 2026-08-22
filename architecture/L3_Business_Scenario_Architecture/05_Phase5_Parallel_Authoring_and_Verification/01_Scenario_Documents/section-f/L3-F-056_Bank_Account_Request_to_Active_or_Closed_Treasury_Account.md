# Bank Account Request to Active or Closed Treasury Account

# 1. Scenario Metadata
**Scenario ID:** L3-F-056  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Treasury & Banking  
**Participating Domains:** B, F  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.1  
**Owner:** Chetan  
**Last Updated:** 2026-08-21  
**Depends On:** `L2-F10_Treasury_and_Banking.md`; `L2-F10_Treasury_and_Banking_Amendment_2026-08-21.md`; authoritative institution/organisation architecture

---
# 2. Scenario Purpose
Govern establishment, activation and eventual closure of a Treasury Account under the canonical F10 lifecycle while keeping external bank infrastructure outside SutraERP ownership.

---
# 3. Business Outcome
The Treasury Account is validly established and either reaches `Active` for permitted Treasury use or, after governed operational use and closure checks, reaches terminal `Closed` with historical evidence preserved.

---
# 4. Scenario Boundary and Scope
## 4.1 Start Boundary
An authorized institutional requirement exists to establish a Treasury Account associated with valid Treasury and banking references.
## 4.2 End Boundary
The requested account has reached `Active`; where the scenario includes later closure, it reaches terminal `Closed` under F10 authority.
## 4.3 In Scope
Request, verification, approval, registration, activation and governed closure.
## 4.4 Out of Scope
External bank-account ownership, banking APIs, payment execution, settlement, ledger posting and technical account provisioning.

---
# 5. Ownership and Participating Domains
F10 is the sole owner of the Treasury Account lifecycle. Section B provides institution/organisation references only. External banking institutions remain authoritative for external bank infrastructure.

---
# 6. Business Actors, Roles and Responsibilities
| Actor / Authority | Responsibility |
|---|---|
| Treasury Account Requestor | Submits the governed account requirement. |
| Treasury Account Verifier | Verifies institution, Treasury, currency and banking-reference completeness. |
| Treasury Account Approval Authority | Approves/rejects establishment and activation. |
| Treasury Account Administrator | Registers and activates after required approvals. |
| Treasury Account Closure Authority | Authorizes `Active → Closed` after closure checks. |

Initiation alone never constitutes approval.

**Authoritative source:** `L2-F10_Treasury_and_Banking_Amendment_2026-08-21.md` — Treasury Account business authorities and lifecycle clarification.

---
# 7. Preconditions and Entry Criteria
Valid institution/Treasury context exists; banking references are traceable; required currency/account context is defined; approval and separation rules are effective.

---
# 8. Trigger
An authorized Treasury Account establishment request is submitted.

---
# 9. Main Business Flow
1. The Requestor submits the business requirement and supporting context.
2. The Verifier confirms institutional, Treasury, currency and bank-reference completeness.
3. The Approval Authority records the governed approval decision.
4. On approval, the Administrator establishes the Treasury Account in `Registered`; F10 publishes `TreasuryAccountRegistered`.
5. After required approval/verification gates, the Administrator activates it: `Registered → Active`; F10 publishes `TreasuryAccountActivated`.
6. The Active account may participate in permitted Treasury operations subject to Treasury restrictions.
7. When closure is requested, the Closure Authority verifies no prohibited pending Treasury activity remains, settlement/reconciliation responsibilities are addressed, references remain traceable and closure authorization is recorded.
8. The account progresses `Active → Closed`; F10 publishes `TreasuryAccountClosed`.
9. Historical identity and evidence remain preserved.

---
# 10. Alternate Business Flows
A rejected establishment request never reaches Registered. Treasury-wide Restricted/Suspended states may prevent use of an Active account but do not become duplicate Treasury Account lifecycle states.

---
# 11. Failure, Interruption and Business Recovery
Missing verification, approval or closure evidence prevents the relevant business transition. Recovery is by completing the governed business prerequisite; no aggregate-version, persistence, concurrency, rollback or publication-recovery mechanism is defined.

---
# 12. Business Rules and Decision Constraints
- Canonical lifecycle is `Registered → Active → Closed`.
- Closed is terminal for operational use.
- Request/verification/approval/closure are independently governable.
- External bank records are never altered by this lifecycle.
- Closure preserves all Treasury references and history.

**Authoritative sources:** `L2-F10_Treasury_and_Banking.md`; `L2-F10_Treasury_and_Banking_Amendment_2026-08-21.md`.

---
# 13. State and Lifecycle Impact
`Registered → Active → Closed`.

**Authoritative source:** `L2-F10_Treasury_and_Banking_Amendment_2026-08-21.md`.

---
# 14. Business Events and Cross-Domain Collaboration
`TreasuryAccountRegistered`, `TreasuryAccountActivated`, `TreasuryAccountClosed` are all published solely by Treasury & Banking.

**Authoritative sources:** `L2-F10_Treasury_and_Banking.md`; `L2-F10_Treasury_and_Banking_Amendment_2026-08-21.md`.

---
# 15. Business Authorization, Approval and Audit Obligations
Preserve request, verification, approval, account identity, activation, closure checks/authority and all lifecycle events. Historical evidence is immutable.

---
# 16. Completion Criteria and Postconditions
The Treasury Account has reached the applicable canonical outcome with required authority/evidence, external bank ownership is unchanged, and no implementation mechanics have been introduced.

---
# 17. Open Issues, Assumptions and Deferred Questions
None material after the 2026-08-21 F10 amendment.

---
**End of Scenario L3-F-056**