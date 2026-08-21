# Overdue Receivable to Governed Dunning Outcome

# 1. Scenario Metadata

**Scenario ID:** L3-F-040  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Accounts Receivable  
**Participating Domains:** C, F, H  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.1  
**Owner:** Chetan  
**Last Updated:** 2026-08-21  
**Depends On:** `L2-F07_Accounts_Receivable_Management.md`; `L2-F07_Accounts_Receivable_Management_Amendment_2026-08-21.md`  
**Referenced By:** None yet  

---

# 2. Scenario Purpose

Govern Accounts Receivable ageing and collection classification when an outstanding Receivable becomes overdue, while keeping communication execution, penalties and source obligations with their canonical owners.

---

# 3. Business Outcome

The overdue Receivable has an authoritative ageing/collection position and any required collection escalation or suspension is governed and traceable under AR policy.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

An outstanding Receivable crosses an applicable due/ageing boundary under effective AR policy.

## 4.2 End Boundary

AR has reviewed and established the current collection classification, escalation or suspension outcome and preserved the supporting evidence.

## 4.3 In Scope

Ageing classification, collection status review, escalation/suspension classification, policy evidence and downstream communication reference where applicable.

## 4.4 Out of Scope

Creating penalties or charges, executing payment collection, changing the source obligation, or designing notification technology.

---

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| Accounts Receivable / F07 | Owns ageing, collection profile/status and AR collection-governance outcome. |
| Student/party lifecycle where applicable / C | Provides authoritative party/student context only. |
| Institutional communication capability / H | May consume the governed collection outcome for communication; does not own AR status. |

---

# 6. Business Actors, Roles and Responsibilities

| Actor / Authority | Responsibility | Boundary |
|---|---|---|
| Collection Review Authority | Reviews ageing and governs AR collection classification/escalation. | Does not execute penalties or source-domain changes. |
| Receivable Review Authority | Reviews collection exceptions affecting AR interpretation. | Corrective financial action remains with its owner. |

**Authoritative source:** `L2-F07_Accounts_Receivable_Management_Amendment_2026-08-21.md` §1.

---

# 7. Preconditions and Entry Criteria

- An active outstanding Receivable exists.
- Due information and effective ageing/collection policy are available.
- Current outstanding balance and collection profile are valid.

---

# 8. Trigger

The Receivable becomes overdue under the applicable AR ageing policy.

---

# 9. Main Business Flow

1. AR evaluates the Receivable against effective ageing criteria.
2. The collection profile is updated with the applicable ageing classification.
3. The Collection Review Authority evaluates priority, escalation and any permitted collection suspension status.
4. Any exception is reviewed without changing the originating obligation.
5. AR records the governed collection position and supporting evidence.
6. Where communication is required, the governed AR outcome is made available to the appropriate communication capability without transferring ownership.

---

# 10. Alternate Business Flows

A disputed or policy-exception receivable may remain under review or collection suspension according to approved AR policy until the governing business issue is resolved.

---

# 11. Failure, Interruption and Business Recovery

Missing or contradictory due/ageing evidence prevents an authoritative collection escalation. AR preserves the current valid position and routes the issue for business review. No technical notification retry or system-recovery behavior is defined.

---

# 12. Business Rules and Decision Constraints

- AR owns ageing and collection classification, not the originating obligation.
- Collection governance does not create late fees or penalties.
- Communication activity does not change AR state by itself.
- Institution-specific ageing buckets, priorities and escalation thresholds remain configuration-governed.

**Authoritative source:** `L2-F07_Accounts_Receivable_Management_Amendment_2026-08-21.md` §§1, 3 and 4; unaffected canonical L2-F07 ageing/collection policy definitions remain authoritative.

---

# 13. State and Lifecycle Impact

This scenario updates the AR-owned Collection Profile/Status using only states/classifications supported by F07 policy. It does not invent a new Receivable lifecycle state.

**Authoritative source:** canonical `L2-F07_Accounts_Receivable_Management.md`, constrained by the ownership and authority rules in the 2026-08-21 amendment §§1, 3 and 4.

---

# 14. Business Events and Cross-Domain Collaboration

Where a material AR change is published, the existing `ReceivableUpdated` fact may communicate the completed Receivable change. No dunning-specific canonical event is invented where L2-F07 has not defined one.

**Authoritative source:** canonical `L2-F07_Accounts_Receivable_Management.md`; the amendment preserves source ownership and prohibits invented event contracts in §§2–5.

---

# 15. Business Authorization, Approval and Audit Obligations

Evidence must preserve Receivable identity, due basis, ageing classification, collection-policy basis, reviewing authority, escalation/suspension decision and any downstream communication reference.

---

# 16. Completion Criteria and Postconditions

- current ageing and collection position is authoritative;
- any escalation/suspension is governed and attributable;
- source obligation ownership is unchanged;
- no penalty or payment execution is implied;
- evidence is reconstructable.

---

# 17. Open Issues, Assumptions and Deferred Questions

Exact institution-specific ageing and dunning thresholds remain configuration policy. No new event name is assumed.

---

**End of Scenario L3-F-040**