# Uncollectible Receivable to Write-Off Decision

# 1. Scenario Metadata

**Scenario ID:** L3-F-042  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Accounts Receivable  
**Participating Domains:** F  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.1  
**Owner:** Chetan  
**Last Updated:** 2026-08-21  
**Depends On:** `L2-F04_Financial_Adjustment_Management.md`; `L2-F07_Accounts_Receivable_Management.md`; `L2-F07_Accounts_Receivable_Management_Amendment_2026-08-21.md`; `L3-F-022`  
**Referenced By:** None yet  

---

# 2. Scenario Purpose

Govern AR assessment and referral of an apparently uncollectible Receivable into the applicable Finance write-off decision process without allowing AR to self-authorize or execute the write-off.

---

# 3. Business Outcome

The Receivable has a governed write-off referral decision supported by collection and outstanding-balance evidence; any actual write-off remains subject to the separately owned Financial Adjustment approval and application process.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

An outstanding Receivable is identified as potentially uncollectible under applicable Finance policy.

## 4.2 End Boundary

AR has either referred the Receivable for write-off governance with complete evidence or declined/deferred the referral with reasons preserved.

## 4.3 In Scope

Uncollectibility evidence, outstanding-position validation, AR review and write-off referral decision.

## 4.4 Out of Scope

Approving or applying the Financial Adjustment, changing the source obligation, accounting posting or Treasury execution.

---

# 5. Ownership and Participating Domains

Accounts Receivable owns the Receivable and referral evidence. Financial Adjustment Management owns any resulting Write-Off Adjustment. The source obligation remains with its canonical source owner.

---

# 6. Business Actors, Roles and Responsibilities

| Actor / Authority | Responsibility | Boundary |
|---|---|---|
| Write-Off Referral Authority | Determines whether AR evidence supports referral to Finance write-off governance. | Does not approve or execute the write-off. |
| Receivable Review Authority | Reviews exceptions in AR outstanding/collection evidence. | Does not alter source-owned facts. |
| Applicable write-off approval authority | Decides the Financial Adjustment outside this scenario. | Owned by the applicable approval governance; Student-Charge write-off requires Board approval. |

**Authoritative source:** `L2-F07_Accounts_Receivable_Management_Amendment_2026-08-21.md` §§1 and 4; write-off approval remains governed by `L2-F04_Financial_Adjustment_Management.md` and its locked amendment.

---

# 7. Preconditions and Entry Criteria

- A valid outstanding Receivable exists.
- Collection/outstanding evidence is available.
- Applicable write-off eligibility policy can be evaluated.
- The source obligation and Receivable lineage are traceable.

---

# 8. Trigger

AR identifies a Receivable as potentially uncollectible and requiring governed write-off assessment.

---

# 9. Main Business Flow

1. AR validates the current outstanding amount and collection history.
2. The Write-Off Referral Authority evaluates applicable eligibility policy and supporting evidence.
3. Any exception is reviewed without changing the source obligation.
4. If referral criteria are satisfied, AR records a write-off referral with Receivable identity, amount, reason and supporting evidence.
5. The referral is handed to the canonical Financial Adjustment/write-off governance.
6. AR preserves its referral decision and awaits an authoritative Finance outcome before changing the Receivable representation.

---

# 10. Alternate Business Flows

If referral criteria are not satisfied, AR retains the existing Receivable position and records the declined/deferred referral reason. Collection governance may continue under L3-F-040.

---

# 11. Failure, Interruption and Business Recovery

Incomplete or contradictory evidence prevents referral approval by AR. The Receivable remains unchanged until the business evidence is corrected or clarified. No technical retry or rollback is defined.

---

# 12. Business Rules and Decision Constraints

- Write-off referral is not write-off approval.
- AR does not create, approve or apply the Financial Adjustment.
- Actual write-off must follow F04 and applicable authority rules.
- Student-Charge write-off requires Board approval under the approved Finance rule.
- Historical Receivable and source-obligation evidence remains traceable.

**Authoritative source:** `L2-F07_Accounts_Receivable_Management_Amendment_2026-08-21.md` §§1, 3 and 4; `L2-F04_Financial_Adjustment_Management.md` plus the locked 2026-08-21 F04 amendment govern the actual adjustment/write-off authority.

---

# 13. State and Lifecycle Impact

This scenario does not itself move the Receivable to `Written Off`. That AR outcome can be recorded only after an authoritative approved/applied Finance write-off fact is received.

**Authoritative source:** canonical `L2-F07_Accounts_Receivable_Management.md`, constrained by the 2026-08-21 amendment §§3–4.

---

# 14. Business Events and Cross-Domain Collaboration

No new write-off-referral event is invented. After an authoritative write-off is applied and AR incorporates that fact, the existing `ReceivableWrittenOff` event may represent the completed AR-owned receivable outcome.

**Authoritative source:** canonical `L2-F07_Accounts_Receivable_Management.md`; event-contract restraint and source ownership are reinforced by the 2026-08-21 amendment §§2–5.

---

# 15. Business Authorization, Approval and Audit Obligations

Evidence must preserve outstanding amount, collection basis, referral authority, referral decision/reason and hand-off reference to the Financial Adjustment process. Actual write-off approval evidence remains owned by its canonical process.

---

# 16. Completion Criteria and Postconditions

The AR referral decision is governed and traceable; the Receivable is not treated as written off merely because referral occurred; and any actual write-off remains separately authorized and owned.

---

# 17. Open Issues, Assumptions and Deferred Questions

No material AR authority blocker remains after the 2026-08-21 F07 amendment.

---

**End of Scenario L3-F-042**