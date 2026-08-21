# Financial Year and Period Establishment to Open Processing Window

# 1. Scenario Metadata
**Scenario ID:** L3-F-051  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Financial Period & Closing  
**Participating Domains:** B, F  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.1  
**Owner:** Chetan  
**Last Updated:** 2026-08-21  
**Depends On:** `L2-F09_Financial_Period_and_Closing.md`; `L2-F09_Financial_Period_and_Closing_Amendment_2026-08-21.md`; authoritative institution/organisation references  
**Governance References:** `L3_Business_Scenario_Template_v2.0.md`

---
# 2. Scenario Purpose
Establish the Finance-owned Financial Year and its governed Financial Periods and open the applicable accounting processing window without duplicating ownership in Accounting & GL.

---
# 3. Business Outcome
A Financial Year is `Open` as the governing annual accounting context and the applicable Financial Period is `Open` for permitted financial processing, with their distinct identities and relationship preserved.

---
# 4. Scenario Boundary and Scope
## 4.1 Start Boundary
A valid institutional requirement exists to establish a new Financial Year and associated Financial Period definitions.
## 4.2 End Boundary
The Financial Year and applicable Financial Period have completed governed opening under F09.
## 4.3 In Scope
Year/period definition, association, readiness and opening authority.
## 4.4 Out of Scope
Journal posting, period close, year close, technical persistence/versioning and calendar ownership outside Finance.

---
# 5. Ownership and Participating Domains
F09 owns both distinct aggregate lifecycles. The Financial Year is not an entity owned by each Financial Period. Institution/organisation identity remains owned by Section B.

---
# 6. Business Actors, Roles and Responsibilities
| Actor / Authority | Responsibility |
|---|---|
| Financial Calendar Administrator | Establishes Financial Year/Period definitions and submits them for opening. |
| Period Opening Authority | Confirms readiness and opens the Financial Period. |

F09 owns year-level opening governance; local role mapping may vary without changing these responsibilities.

**Authoritative source:** `L2-F09_Financial_Period_and_Closing_Amendment_2026-08-21.md` §4.

---
# 7. Preconditions and Entry Criteria
Institution/organisation references are valid; year and period boundaries are defined; periods reference exactly one Financial Year; required Finance policy is effective.

---
# 8. Trigger
A new governed Finance accounting cycle must be established for the institution.

---
# 9. Main Business Flow
1. The Financial Calendar Administrator establishes the Financial Year identity and annual boundary.
2. Associated Financial Period definitions are established, each referencing exactly one Financial Year.
3. F09 validates non-contradictory year/period association and applicable opening prerequisites.
4. The Financial Year progresses `Planned → Open`; F09 publishes `FinancialYearOpened`.
5. The Period Opening Authority confirms the applicable period is ready.
6. The Financial Period progresses `Planned/Registered → Open` under F09 governance.
7. The open year/period context becomes available to Finance capabilities; F05 consumes the period state for posting eligibility.

---
# 10. Alternate Business Flows
A period not ready for opening remains in its preparation boundary while the Financial Year may remain Open if governance permits. No third preparation state is invented.

---
# 11. Failure, Interruption and Business Recovery
Invalid boundaries, ownership references or opening prerequisites prevent the affected lifecycle transition. Business correction occurs under F09 governance; no aggregate-version, persistence, concurrency, rollback or event-delivery recovery is defined.

---
# 12. Business Rules and Decision Constraints
- Financial Year and Financial Period are distinct F09 aggregate roots.
- One Financial Year may associate with many Financial Periods.
- Every Financial Period references exactly one Financial Year.
- F05 consumes period/year state; it does not own these lifecycles.
- Opening requires the applicable F09 authority.

**Authoritative source:** `L2-F09_Financial_Period_and_Closing_Amendment_2026-08-21.md` §§1, 4 and 5.

---
# 13. State and Lifecycle Impact
Financial Year: `Planned → Open`.  
Financial Period: `Planned/Registered → Open`.

**Authoritative source:** `L2-F09_Financial_Period_and_Closing_Amendment_2026-08-21.md` §§2–3.

---
# 14. Business Events and Cross-Domain Collaboration
`FinancialYearOpened` is the canonical F09 annual opening fact. Period lifecycle facts remain F09-owned. Section B provides reference context only.

**Authoritative source:** `L2-F09_Financial_Period_and_Closing_Amendment_2026-08-21.md` §§2 and 5.

---
# 15. Business Authorization, Approval and Audit Obligations
Preserve year/period identities, boundaries, associations, administrator, opening authority, decisions and resulting lifecycle states.

---
# 16. Completion Criteria and Postconditions
Year is Open; applicable period is Open; ownership is unambiguous; Finance consumers can rely on F09 lifecycle facts; no implementation mechanics are introduced.

---
# 17. Open Issues, Assumptions and Deferred Questions
The `Planned/Registered` label normalization remains an upstream terminology-cleanup item but does not create a third state or ownership ambiguity.

---
**End of Scenario L3-F-051**