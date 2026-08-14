# L2 Canonical Section Gap Report

> **Follow-up — 2026-08-13:** The gaps listed here remain a historical repository-wide snapshot. The approved targeted maintenance pass has closed all listed crosswalk gaps in L2-J02 through L2-J08 and L2-J10. Other files in this report were outside that approval and were not changed by the Dhruv audit application.

**Finding:** INT-018 | **Date:** 2026-08-04 | **Files:** 107

Volume 3 section 18 requires eleven sections in every L2 document. The Canonical
Section Crosswalk added to each file records where each required section lives.
This report lists the sections that have **no counterpart at all** and therefore
need authoring. Nothing was invented to fill these gaps.

## Summary

| Required section | Files missing it |
|---|---:|
| Purpose | 0 |
| Scope | 4 |
| Actors | 43 |
| Entities | 15 |
| Business Rules | 0 |
| Workflow | 0 |
| State Machine | 0 |
| Contracts | 30 |
| Events | 0 |
| Dependencies | 8 |
| Locked Decisions | 53 |

## Detail

### Scope — 4 files

- L2-J03_Purchase_Requisition_and_Approval_Workflows.md
- L2-J04_Purchase_Order_Management.md
- L2-J05_Goods_Receipt_Inspection_and_Acceptance.md
- L2-J06_Inventory_and_Warehouse_Management.md

### Actors — 43 files

- L2-A00_Platform_Foundation_Architecture.md
- L2-A01_Platform_Tenant_Foundation_Architecture.md
- L2-A03_Authentication_Workflows.md
- L2-A05_Workflow_Engine_Workflows.md
- L2-A09_Master_Data_Workflows.md
- L2-A10_Operational_Readiness_Workflows.md
- L2-B00_Institution_and_Structure_Review.md
- L2-B01_Institution_Workflow.md
- L2-B02_Organisation_Workflows.md
- L2-B05_Academic_Calendar_Workflows.md
- L2-C00_Student_Lifecycle_Review.md
- L2-D00_Academic_Delivery_Review.md
- L2-D04_Teaching_Session_and_Attendance_Workflows.md
- L2-D07_Internal_Assessment_Workflows.md
- L2-F01_Student_Charge_and_Fee_Management.md
- L2-F02_Payment_Management_Architecture.md
- L2-F03_Refund_Management.md
- L2-F04_Financial_Adjustment_Management.md
- L2-F06_Budget_and_Cost_Center_Management.md
- L2-F07_Accounts_Receivable_Management.md
- L2-F08_Accounts_Payable_Management.md
- L2-F09_Financial_Period_and_Closing.md
- L2-F10_Treasury_and_Banking.md
- L2-F11_Scholarship_and_Financial_Aid.md
- L2-F12_Asset_Financial_Accounting_Workflows.md
- L2-F13_Financial_Reporting_and_Analytics_Workflows.md
- L2-F14_Procurement_Finance_Consumption_Workflows.md
- L2-G10_Infrastructure_Management_Workflows.md
- L2-H00_Institutional_Services_and_Intelligence_Architecture_Governance.md
- L2-H01_APAAR_and_DigiLocker_Workflows.md
- L2-H02_Notification_Workflows.md
- L2-H03_Enterprise_Directory_and_Search_Workflows.md
- L2-H04_Placement_Workflows.md
- L2-H07_Enterprise_Dashboard_Workflows.md
- L2-H08_NEP_NAAC_AISHE_Reporting_Workflows.md
- L2-I00_Examination_Review.md
- L2-J02_Vendor_and_Supplier_Management_Workflows.md
- L2-J03_Purchase_Requisition_and_Approval_Workflows.md
- L2-J04_Purchase_Order_Management.md
- L2-J05_Goods_Receipt_Inspection_and_Acceptance.md
- L2-J06_Inventory_and_Warehouse_Management.md
- L2-J08_Asset_Lifecycle_Management.md
- L2-J10_Disposal_Scrap_and_Write_Off.md

### Entities — 15 files

- L2-A00_Platform_Foundation_Architecture.md
- L2-B01_Institution_Workflow.md
- L2-I03_Examination_Planning_and_Scheduling_Workflows.md
- L2-I04_Examination_Registration_Workflows.md
- L2-I05_Hall_Ticket_Management_Workflows.md
- L2-I06_Practical_and_Viva_Examination_Conduct_Workflows.md
- L2-I07_Theory_Examination_Conduct_Workflows.md
- L2-I08_Marks_Collection_and_Evaluation_Workflows.md
- L2-I09_Result_Processing_and_Computation_Workflows.md
- L2-I10_Result_Publication_Workflows.md
- L2-I11_Revaluation_and_Rechecking_Workflows.md
- L2-I12_Supplementary_Backlog_and_Improvement_Examination_Execution_Workflows.md
- L2-J02_Vendor_and_Supplier_Management_Workflows.md
- L2-J06_Inventory_and_Warehouse_Management.md
- L2-J08_Asset_Lifecycle_Management.md

### Contracts — 30 files

- L2-B00_Institution_and_Structure_Review.md
- L2-B01_Institution_Workflow.md
- L2-C00_Student_Lifecycle_Review.md
- L2-F06_Budget_and_Cost_Center_Management.md
- L2-F12_Asset_Financial_Accounting_Workflows.md
- L2-F14_Procurement_Finance_Consumption_Workflows.md
- L2-H09_Accreditation_and_Compliance_Workflows.md
- L2-I00_Examination_Review.md
- L2-I01_Examination_Policy_and_Configuration_Workflows.md
- L2-I02_Examination_Session_Management_Workflows.md
- L2-I03_Examination_Planning_and_Scheduling_Workflows.md
- L2-I04_Examination_Registration_Workflows.md
- L2-I05_Hall_Ticket_Management_Workflows.md
- L2-I06_Practical_and_Viva_Examination_Conduct_Workflows.md
- L2-I07_Theory_Examination_Conduct_Workflows.md
- L2-I08_Marks_Collection_and_Evaluation_Workflows.md
- L2-I09_Result_Processing_and_Computation_Workflows.md
- L2-I10_Result_Publication_Workflows.md
- L2-I11_Revaluation_and_Rechecking_Workflows.md
- L2-I12_Supplementary_Backlog_and_Improvement_Examination_Execution_Workflows.md
- L2-I13_Academic_Completion_CGPA_and_Completion_Evidence_Workflows.md
- L2-I14_Malpractice_and_Examination_Committee_Workflows.md
- L2-I15_Examination_Exception_and_Contingency_Workflows.md
- L2-I16_Examination_Session_Closure_and_Archival_Workflows.md
- L2-J02_Vendor_and_Supplier_Management_Workflows.md
- L2-J03_Purchase_Requisition_and_Approval_Workflows.md
- L2-J04_Purchase_Order_Management.md
- L2-J05_Goods_Receipt_Inspection_and_Acceptance.md
- L2-J06_Inventory_and_Warehouse_Management.md
- L2-J08_Asset_Lifecycle_Management.md

### Dependencies — 8 files

- L2-D00_Academic_Delivery_Review.md
- L2-D05_Academic_Monitoring_Workflows.md
- L2-F12_Asset_Financial_Accounting_Workflows.md
- L2-F14_Procurement_Finance_Consumption_Workflows.md
- L2-H09_Accreditation_and_Compliance_Workflows.md
- L2-J02_Vendor_and_Supplier_Management_Workflows.md
- L2-J08_Asset_Lifecycle_Management.md
- L2-J10_Disposal_Scrap_and_Write_Off.md

### Locked Decisions — 53 files

- L2-A08_Module_Registry_Configuration_Workflows.md
- L2-B00_Institution_and_Structure_Review.md
- L2-B01_Institution_Workflow.md
- L2-B03_Campus_Workflows.md
- L2-C00_Student_Lifecycle_Review.md
- L2-D00_Academic_Delivery_Review.md
- L2-D07_Internal_Assessment_Workflows.md
- L2-F01_Student_Charge_and_Fee_Management.md
- L2-F02_Payment_Management_Architecture.md
- L2-F03_Refund_Management.md
- L2-F04_Financial_Adjustment_Management.md
- L2-F05_Accounting_and_General_Ledger_Management.md
- L2-F06_Budget_and_Cost_Center_Management.md
- L2-F07_Accounts_Receivable_Management.md
- L2-F08_Accounts_Payable_Management.md
- L2-F09_Financial_Period_and_Closing.md
- L2-F10_Treasury_and_Banking.md
- L2-F12_Asset_Financial_Accounting_Workflows.md
- L2-F14_Procurement_Finance_Consumption_Workflows.md
- L2-G00_Campus_Operations_Review_and_Governance.md
- L2-G01_Facilities_Management_Workflows.md
- L2-G02_Laboratory_Management_Workflows.md
- L2-G04_Transport_Management_Workflows.md
- L2-G05_Library_Management_Workflows.md
- L2-G06_Institutional_Health_Services_and_Medical_Workflows.md
- L2-G07_Security_Operations_Workflows.md
- L2-G08_Housekeeping_Operations_and_Service_Governance_Workflows.md
- L2-G10_Infrastructure_Management_Workflows.md
- L2-H04_Placement_Workflows.md
- L2-H06_Alumni_Workflows.md
- L2-H09_Accreditation_and_Compliance_Workflows.md
- L2-H10_External_Education_Integration_Workflows.md
- L2-I00_Examination_Review.md
- L2-I01_Examination_Policy_and_Configuration_Workflows.md
- L2-I02_Examination_Session_Management_Workflows.md
- L2-I03_Examination_Planning_and_Scheduling_Workflows.md
- L2-I04_Examination_Registration_Workflows.md
- L2-I05_Hall_Ticket_Management_Workflows.md
- L2-I06_Practical_and_Viva_Examination_Conduct_Workflows.md
- L2-I07_Theory_Examination_Conduct_Workflows.md
- L2-I08_Marks_Collection_and_Evaluation_Workflows.md
- L2-I09_Result_Processing_and_Computation_Workflows.md
- L2-I10_Result_Publication_Workflows.md
- L2-I11_Revaluation_and_Rechecking_Workflows.md
- L2-I12_Supplementary_Backlog_and_Improvement_Examination_Execution_Workflows.md
- L2-I13_Academic_Completion_CGPA_and_Completion_Evidence_Workflows.md
- L2-I16_Examination_Session_Closure_and_Archival_Workflows.md
- L2-J02_Vendor_and_Supplier_Management_Workflows.md
- L2-J03_Purchase_Requisition_and_Approval_Workflows.md
- L2-J04_Purchase_Order_Management.md
- L2-J05_Goods_Receipt_Inspection_and_Acceptance.md
- L2-J06_Inventory_and_Warehouse_Management.md
- L2-J08_Asset_Lifecycle_Management.md
