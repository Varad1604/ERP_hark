# ADR-B18-W4-J05-EVENT-001 — Receipt variance event vocabulary reconciliation

**Date:** 2026-10-03  
**Status:** Lead-authorized proposal; Section J/A06 contract review pending  
**Basis:** Architecture main `bb8d9ebdfe4487fb2b3d935d6fa95860f8eb8ca7`. The Lead authorized fixing the verified inconsistency in the current SutraERP project conversation.

## Verified conflict

Locked L2-J00 §27.7 lists `GoodsReceiptVarianceRecorded` for J05 receipt quantity/specification variance. Locked specialized L2-J05 §6 lists `ReceiptVarianceRecorded`. Locked L3-J-049/-050, Review L4-J-049/-050, and F14/L3-F-081 consume or name `ReceiptVarianceRecorded`. Both labels refer to receipt variance with J05 ownership, but there is no inspected approved active A06 event-definition export establishing aliasing, payload compatibility, or a deployed version.

## Proposed patch decision

Use `ReceiptVarianceRecorded` as the *documentary canonical name* for J05 receipt variance/corrective facts, consistent with the specialized locked workflow and downstream scenario vocabulary. Treat the L2-J00 alternate `GoodsReceiptVarianceRecorded` entry as a candidate historical vocabulary discrepancy, **not** an independently publishable second event or an automatically supported runtime alias. Preserve J05 sole business publisher, non-destructive supplemental corrections, A04 authorization and independent J06/J08/Finance consumers.

**Scope of authorization:** The Lead authorized resolution work, not evidence of an existing registry entry or the right to impersonate Chetan/A06 contract assent. Under Volume 5 proposal/review/decision/update sequence, this ADR and its companion versioned amendment are **pending applicable Section J and A06 event-definition review**. Do not edit the existing Locked L2-J00 in place or mark the patch active before those gates. Upon review and formal approval, the amendment can state that its single catalog row supersedes the conflicting wording in locked L2-J00, while leaving original history intact.

## Explicit verification and operational gate

A06 must present the actually approved active Event Definition (name, owner, version, schema and compatibility/consumer references). If that definition uses the alternative name or does not exist, return to the governed A06 event proposal/version process rather than silently changing a deployed contract. No runtime publishing, alias, new event registration, client command, API, permission or amount/tolerance policy is authorized by this documentation.

## Impact and dependent decisions

- J04/J05: replacement/excess eligibility, quantitative custody/unit policy and safe concurrent issued-PO-version admission remain open in Architecture #144.
- J05: authentic A04/SoD and approved `ReceiptVarianceRecorded` *active schema/payload/version* still required for positive finalized correction.
- J06/J08/F14: verify individual consumer compatibility before A06 activation or any changed definition.
- Finance: F14 recognized-liability revision and F08 suspended-origin release semantics remain separate unresolved Architecture #145 decisions.
- Platform: permanent NOT_APPLIED source-owner verifier and durable intent fence remain unresolved Architecture #146 decisions.

**Current disposition:** Documentary preferred name identified and exact conflict isolated; runtime registry, positive production correction and cross-domain compensation **NOT VERIFIED / BLOCKED**.
