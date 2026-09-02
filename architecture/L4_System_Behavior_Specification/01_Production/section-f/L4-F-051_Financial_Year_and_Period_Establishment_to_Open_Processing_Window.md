# Financial Year and Period Establishment to Open Processing Window

## 1. Specification Metadata

**Specification ID:** L4-F-051  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Financial Period & Closing  
**Realizes Scenario:** L3-F-051  
**Primary Business Owner:** Financial Period & Closing  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Chetan  
**Last Updated:** 2026-08-30  
**Depends On:** `L3-F-051_Financial_Year_and_Period_Establishment_to_Open_Processing_Window.md`; `L2-F09_Financial_Period_and_Closing.md`; `L2-F09_Financial_Period_and_Closing_Amendment_2026-08-21.md`; authoritative institution/organisation references  
**Referenced By:** None yet  

---

## 2. Behavioral Objective & Boundary

SutraERP shall establish the Finance-owned Financial Year and associated Financial Periods as distinct F09 aggregates and open the applicable accounting processing window under governed authority.

The behavior begins when a valid institutional requirement exists to establish a new Financial Year and associated Financial Period definitions. It ends when the Financial Year is `Open` and the applicable Financial Period is `Open` under F09 governance.

This specification does not post Journals, close periods, close the year, own the institutional calendar outside Finance, or define persistence/versioning mechanics.

---

## 3. Required Business Context

The behavior requires:

- valid institution/organisation references;
- defined Financial Year boundary;
- defined Financial Period boundaries;
- each Financial Period referencing exactly one Financial Year;
- applicable Finance policy;
- Financial Calendar Administrator;
- Period Opening Authority.

Financial Year and Financial Period are distinct F09 aggregate roots. One Financial Year may associate with many Financial Periods. Section B retains institution/organisation identity ownership.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-F-051-BH-001 | The system shall permit the Financial Calendar Administrator to establish the Financial Year identity and annual boundary. | L3-F-051 §§6, 9 step 1 |
| L4-F-051-BH-002 | The system shall establish Financial Period definitions as distinct aggregates, each referencing exactly one Financial Year. | L3-F-051 §§5, 7, 9 step 2, 12 |
| L4-F-051-BH-003 | The system shall prevent contradictory year/period associations from progressing to opening. | L3-F-051 §§7, 9 step 3, 11 |
| L4-F-051-BH-004 | The system shall preserve the rule that a Financial Year is not an entity contained separately inside each Financial Period aggregate. | L3-F-051 §§5, 12 |
| L4-F-051-BH-005 | When year-level opening prerequisites are satisfied, the system shall move the Financial Year from `Planned` to `Open`. | L3-F-051 §§3, 9 step 4, 13 |
| L4-F-051-BH-006 | When `Planned → Open` completes, the system shall publish `FinancialYearOpened` with F09 as canonical publisher. | L3-F-051 §§9 step 4, 14 |
| L4-F-051-BH-007 | The system shall require Period Opening Authority confirmation before the applicable Financial Period may open. | L3-F-051 §§6, 9 step 5, 12 |
| L4-F-051-BH-008 | When period opening prerequisites are satisfied, the system shall move the applicable Financial Period from `Planned/Registered` to `Open`. | L3-F-051 §§9 step 6, 13 |
| L4-F-051-BH-009 | The system shall not invent a third preparation state from the `Planned/Registered` terminology. | L3-F-051 §§10, 17 |
| L4-F-051-BH-010 | Once the year/period context is open, the system shall expose authoritative F09 lifecycle facts to Finance consumers. | L3-F-051 §§3, 9 step 7, 16 |
| L4-F-051-BH-011 | F05 shall consume the Financial Period state for posting eligibility without owning or changing the F09 lifecycle. | L3-F-051 §§2, 9 step 7, 12, 16 |
| L4-F-051-BH-012 | Section B institution/organisation references shall be consumed as context without transfer of identity ownership to F09. | L3-F-051 §§5, 14 |

---

## 5. Cross-Domain & Event Behavior

`FinancialYearOpened` is the canonical F09 annual opening fact. Financial Period lifecycle facts remain F09-owned and published. F05 consumes the year/period state for posting eligibility. Section B provides institutional reference context only.

---

## 6. Failure / Rejection / Continuation

Invalid boundaries, invalid ownership references, or unmet opening prerequisites shall prevent the affected lifecycle transition. A Financial Period may remain at its existing preparation boundary while the Financial Year remains `Open` where governance permits. No technical concurrency, persistence, rollback, or event-delivery recovery is specified.

---

## 7. Audit & Evidence Behavior

The system shall preserve Financial Year identity and boundary, Financial Period identities and boundaries, year-period associations, institution/organisation references, Financial Calendar Administrator participation, Period Opening Authority decision, resulting states, and `FinancialYearOpened`.

---

## 8. Observable Completion Conditions

Completion requires the Financial Year to be `Open`, the applicable Financial Period to be `Open`, year/period identities and relationship to be unambiguous, F09 ownership to remain intact, and Finance consumers to be able to rely on F09 lifecycle facts.

---

## 9. Traceability & Provenance

**Parent:** `L3-F-051` — Financial Year and Period Establishment to Open Processing Window  
**Primary verification source:** `L2-F09` — Financial Period & Closing  
**Locked amendment:** `L2-F09_Financial_Period_and_Closing_Amendment_2026-08-21.md`

The locked amendment verifies the two distinct aggregates, year/period relationship, authorities, Financial Year lifecycle, Financial Period lifecycle, and F09 publisher ownership.

---

## 10. Open Issues / Deferred Design

`Planned/Registered` remains an upstream terminology-normalization item. This L4 does not resolve it or invent another state.

API/interface design, persistence, versioning, technical calendar storage, event transport, infrastructure, and UI/UX remain deferred.

---

**Draft Validation Classification:** Prepared as an L4 Draft for architecture review.
