# Vendor Invoice to Approved Payable Liability

# 1. Scenario Metadata
**Scenario ID:** L3-F-044  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Accounts Payable  
**Participating Domains:** F, J  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.1  
**Owner:** Chetan  
**Last Updated:** 2026-08-21  
**Depends On:** `L2-F08_Accounts_Payable_Management.md`; `L2-F08_Accounts_Payable_Management_Amendment_2026-08-21.md`; `L2-F14_Procurement_Finance_Consumption_Workflows_Amendment_2026-08-21.md`; `L2-J02_Vendor_and_Supplier_Management_Workflows.md`; `L2-J04_Purchase_Order_Management.md`; `L2-J05_Goods_Receipt_Inspection_and_Acceptance.md`

---
# 2. Scenario Purpose
Govern the Finance hand-off from accepted procurement evidence to a validated canonical Payable without transferring procurement or supplier ownership to Finance.

---
# 3. Business Outcome
A Finance-recognized vendor liability has been registered and validated as the canonical Accounts Payable representation, traceable to authoritative Section J procurement facts.

---
# 4. Scenario Boundary and Scope
## 4.1 Start Boundary
Authoritative Section J procurement facts and vendor invoice evidence are available for Finance matching and approval.
## 4.2 End Boundary
The Payable is registered and validated after `VendorLiabilityRecognized`.
## 4.3 In Scope
Finance-side matching/approval, liability recognition, AP registration/validation and traceability.
## 4.4 Out of Scope
Supplier identity, procurement approval, PO/receipt ownership, Treasury settlement, ledger posting and technical integration.

---
# 5. Ownership and Participating Domains
Section J owns supplier and procurement facts. F14 owns Finance invoice matching/approval and vendor-liability recognition. F08 owns the Payable and its lifecycle.

---
# 6. Business Actors, Roles and Responsibilities
Finance actors perform the F14 invoice-governance responsibilities already authorized by the procurement-finance workflow; AP registration/validation responsibilities remain with Accounts Payable. This scenario does not invent additional role names beyond the authoritative sources.

---
# 7. Preconditions and Entry Criteria
- authoritative procurement facts exist;
- vendor invoice evidence is traceable;
- required matching and Finance approval rules are effective;
- supplier identity remains authoritative in Section J.

---
# 8. Trigger
Finance receives vendor invoice evidence together with the required procurement-owned facts.

---
# 9. Main Business Flow
1. F14 consumes authoritative Section J procurement facts.
2. F14 completes the required vendor-invoice match and publishes `VendorInvoiceMatched`.
3. Finance-side invoice governance completes and F14 publishes `VendorInvoiceApproved`.
4. F14 recognizes the vendor financial obligation and publishes `VendorLiabilityRecognized`.
5. F08 consumes `VendorLiabilityRecognized` as the single authoritative AP hand-off.
6. F08 creates the canonical Payable and publishes `PayableRegistered`.
7. F08 validates the Payable and publishes `PayableValidated`.
8. Procurement, supplier, liability and Payable lineage is preserved without ownership transfer.

---
# 10. Alternate Business Flows
If matching or Finance approval fails, liability recognition and AP registration do not occur. The matter remains in its owning Finance/procurement governance path.

---
# 11. Failure, Interruption and Business Recovery
An incomplete or contradictory procurement/invoice fact prevents progression. No event is skipped or treated as a synonym to force AP creation. L3 defines no technical recovery mechanism.

---
# 12. Business Rules and Decision Constraints
- `VendorInvoiceApproved` is published by F14, not Section J.
- `VendorLiabilityRecognized` is the AP hand-off.
- `PayableRegistered` and `PayableValidated` are F08-owned distinct facts.
- No listed event is an alias for another.
- Vendor/supplier and procurement ownership remain in Section J.

**Authoritative sources:** `L2-F08_Accounts_Payable_Management_Amendment_2026-08-21.md`; `L2-F14_Procurement_Finance_Consumption_Workflows_Amendment_2026-08-21.md`; applicable procurement facts from `L2-J02`, `L2-J04`, and `L2-J05`.

---
# 13. State and Lifecycle Impact
F14 establishes the Finance liability fact; F08 then establishes and validates the Payable. This scenario does not redefine Section J lifecycles.

**Authoritative source:** `L2-F08_Accounts_Payable_Management_Amendment_2026-08-21.md`.

---
# 14. Business Events and Cross-Domain Collaboration
`Section J facts → VendorInvoiceMatched → VendorInvoiceApproved → VendorLiabilityRecognized → PayableRegistered → PayableValidated` is the canonical sequence.

**Authoritative source:** `L2-F14_Procurement_Finance_Consumption_Workflows_Amendment_2026-08-21.md` together with `L2-F08_Accounts_Payable_Management_Amendment_2026-08-21.md`.

---
# 15. Business Authorization, Approval and Audit Obligations
Evidence must preserve procurement references, invoice evidence, matching result, Finance approval, recognized liability, Payable identity, validation outcome and each event publisher.

---
# 16. Completion Criteria and Postconditions
The Payable is registered and validated from `VendorLiabilityRecognized`; event ownership is unambiguous; Section J ownership remains intact; downstream AP processing may proceed.

---
# 17. Open Issues, Assumptions and Deferred Questions
No material event-publisher or hand-off blocker remains after the 2026-08-21 F08/F14 amendments.

---
**End of Scenario L3-F-044**