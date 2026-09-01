# L3 Controlled Authority Amendment Decision — G200, G212, G214 — 2026-09-01

**Version:** 1.0.0  
**Status:** Architecture Approved  
**Architectural Level:** L3 — Business Scenario Architecture Governance  
**Domain:** Campus Operations — Security Operations (G07)  
**Decision Date:** 2026-09-01  
**Affected Scenarios:** `L3-G-200`, `L3-G-212`, `L3-G-214`  
**Change Authorization:** Explicit authorization to perform the upstream document corrections was provided in the SutraERP project review session on 2026-09-01.  
**Governing Baseline:** `L3_Final_Corpus_Lock_Decision_1084_2026-08-29.md`; `Architecture_Level_Validation_Standards.md`; `L2-G07_Security_Operations_Workflows.md`

---

## 1. Finding

A downstream L4 parent-readiness review identified internal business-authority contradictions in three Locked L3 Security Operations scenarios:

- `L3-G-200`: `Complainant / Witness` was assigned executive approval and exception-waiver authority despite the scenario assigning supervisory authorization to the Investigating Security Officer and keeping disciplinary/statutory case disposition outside Security Operations proceedings.
- `L3-G-212`: `Escort Requestor` was assigned executive approval and exception-waiver authority despite the scenario assigning supervisory authorization to the Security Dispatch Supervisor and defining the requestor's relevant completion participation as acceptance of the alternative safety arrangement.
- `L3-G-214`: `Platform Access Administrator` was assigned executive approval and exception-waiver authority despite the scenario assigning the business delegation decision to the Chief Security Officer (Grantor). This also risked confusing Platform authorization administration with Security Operations business authority.

These findings made the affected parents materially ambiguous for canonical L4 derivation.

---

## 2. Impact Assessment

The defects are limited to actor and business-authority statements. They do **not** require changes to:

- scenario IDs or titles;
- canonical owner (`Security Operations (G07)`);
- scenario start or end boundaries;
- business outcomes;
- aggregate ownership;
- lifecycle states or transitions;
- business rules;
- published or consumed business events;
- canonical event publishers;
- dependency direction; or
- participating-domain ownership.

The dependent L4 specifications affected by the ambiguity are:

- `L4-G-200`;
- `L4-G-212`;
- `L4-G-214`.

Unaffected Locked L3 scenarios remain frozen.

---

## 3. Controlled Amendment Decision

The following corrections are approved:

### 3.1 L3-G-200

- `Complainant / Witness` is retained only as an evidence-providing participant.
- Unsupported executive approval, exception-waiver, and policy-sign-off authority is removed from that role.
- Existing Investigating Security Officer supervisory authority is preserved.
- Disciplinary/statutory disposition remains outside the execution of downstream disciplinary or judicial proceedings as already bounded by the scenario.

### 3.2 L3-G-212

- `Escort Requestor` is retained as the requesting party and as the party confirming acceptance of the verified alternative safety arrangement.
- Unsupported executive approval, exception-waiver, and policy-sign-off authority is removed from that role.
- Existing Security Dispatch Supervisor supervisory authorization is preserved.

### 3.3 L3-G-214

- `Platform Access Administrator` is removed as a business actor from the L3 authority model because the scenario does not establish that platform-administration role as a business delegation authority.
- Unsupported executive approval, exception-waiver, and policy-sign-off authority is removed.
- Existing Chief Security Officer (Grantor) business delegation authority is preserved.
- Registrar / Appointing Authority participation remains verification / coordination.
- Platform Foundation retains its upstream infrastructure responsibilities without acquiring the Security Operations business decision.

---

## 4. Successor-State and Versioning Decision

The three affected L3 scenario files advance from version `1.0` to `1.1`, retain `Status: Locked`, and carry `Last Updated: 2026-09-01`.

This amendment does not reopen or renumber the L3 corpus. It establishes corrected Locked successor content for only the three affected scenario identities.

---

## 5. Revalidation

The amended scenarios were revalidated against the applicable architecture-level requirements with the following result:

| Scenario | Ownership | Authority Integrity | Lifecycle / State | Events / Publisher | Boundary | Result |
|---|---|---|---|---|---|---|
| `L3-G-200` | Preserved | Corrected | Preserved | Preserved | Preserved | PASS |
| `L3-G-212` | Preserved | Corrected | Preserved | Preserved | Preserved | PASS |
| `L3-G-214` | Preserved | Corrected | Preserved | Preserved | Preserved | PASS |

No new business authority, lifecycle state, business rule, event, publisher, aggregate owner, or cross-domain ownership transfer is introduced by this amendment.

---

## 6. Downstream L4 Consequence

The parent-authority blockers in `L4-G-200`, `L4-G-212`, and `L4-G-214` are resolved by the corrected Locked L3 successor content.

The three L4 files may therefore remove their upstream-authority blocker notes while continuing to preserve the same behavioral boundaries, states, rules, events, publishers, and canonical ownership.

---

## 7. Decision Statement

**CONTROLLED L3 AMENDMENT:** APPROVED  
**AFFECTED L3 SCENARIOS:** 3  
**CORRECTED SUCCESSOR STATUS:** LOCKED  
**L3 CORPUS SIZE:** UNCHANGED — 1,084  
**AFFECTED L4 PARENT-READINESS BLOCKERS:** RESOLVED  
**UNRELATED LOCKED L3 SCENARIOS:** UNCHANGED
