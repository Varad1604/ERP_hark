# SutraERP Architecture Index

This index lists all current and scheduled architectural documentation, mapping ownership, versions, status, and dependency layers.

| Document | Owner | Version | Status | Dependencies |
| :--- | :--- | :--- | :--- | :--- |
| [SutraERP_Architecture_Workflow_Context_Baseline.md](file:///g:/Pravaha/ERP_Planning/docs/baseline/SutraERP_Architecture_Workflow_Context_Baseline.md) | Dhruv | 1.0 | Locked | None |
| [SutraERP_Level2_Collaboration_Guide_v1.1.md](file:///g:/Pravaha/ERP_Planning/docs/collaboration/SutraERP_Level2_Collaboration_Guide_v1.1.md) | Dhruv | 1.1 | Locked | Baseline |
| [SutraERP_Level_1_Architecture_Locked.md](file:///g:/Pravaha/ERP_Planning/docs/level-1/SutraERP_Level_1_Architecture_Locked.md) | Dhruv | 1.0 | Locked | Baseline |
| [Level 2 matrix.md](file:///g:/Pravaha/ERP_Planning/docs/level-2/Level%202%20matrix.md) | Dhruv / Pallav | 1.0 | Draft | L1 Architecture |
| [DEPENDENCY_MAP.md](file:///g:/Pravaha/ERP_Planning/DEPENDENCY_MAP.md) | Dhruv | 1.0 | Locked | Baseline, L1 Architecture |
| **Section A — Platform Foundation** | | | | |
| L2-A01 — Platform & Tenant Workflows | Dhruv | 0.1 | Draft | L1 Architecture |
| L2-A02 — Person & Identity Workflows | Dhruv | 0.1 | Draft | L2-A01 |
| L2-A03 — Authentication Workflows | Dhruv | 0.1 | Draft | L2-A02 |
| L2-A04 — Authorization & Policy Workflows | Dhruv | 0.1 | Draft | L2-A03 |
| L2-A05 — Workflow Engine Workflows | Dhruv | 0.1 | Draft | L1 Architecture |
| L2-A06 — Event Infrastructure Workflows | Dhruv | 0.1 | Draft | L1 Architecture |
| L2-A07 — Audit Infrastructure Workflows | Dhruv | 0.1 | Draft | L1 Architecture |
| L2-A08 — Module Registry Workflows | Dhruv | 0.1 | Draft | L2-A01 |
| L2-A09 — Master Data Workflows | Dhruv | 0.1 | Draft | L1 Architecture |
| L2-A00 — Platform Foundation Review | Dhruv | 0.1 | Draft | All Section A |
| **Section B — Institution & Structure** | | | | |
| L2-B01 — Institution Workflows | Dhruv | 0.1 | Draft | L1 Architecture |
| L2-B02 — Organisation Workflows | Dhruv | 0.1 | Draft | L2-B01 |
| L2-B03 — Campus & Infrastructure Workflows | Dhruv | 0.1 | Draft | L2-B01 |
| L2-B04 — Academic Structure Workflows | Dhruv | 0.1 | Draft | L2-B01 |
| L2-B05 — Academic Year & Period Workflows | Dhruv | 0.1 | Draft | L2-B04 |
| L2-B06 — Operational Readiness Workflows | Dhruv | 0.1 | Draft | All Section B |
| L2-B00 — Institution & Structure Review | Dhruv | 0.1 | Draft | All Section B |
| **Section C — Student Lifecycle** | | | | |
| L2-C01 — Admissions Workflows | Pallav | 0.1 | Draft | Section A, Section B |
| L2-C02 — Student Lifecycle & Activation Workflows | Pallav | 0.1 | Draft | L2-C01 |
| L2-C03 — Student Academic Registration Workflows | Pallav | 0.1 | Draft | L2-C02, Section D |
| L2-C04 — Student Academic Progression Workflows | Pallav | 0.1 | Draft | L2-C03 |
| L2-C05 — Graduation & Student Exit Workflows | Pallav | 0.1 | Draft | L2-C04 |
| L2-C06 — Guardian Workflows | Pallav | 0.1 | Draft | L2-A02 |
| L2-C07 — Scholarship Workflows | Pallav | 0.1 | Draft | L2-C02 |
| L2-C00 — Student Lifecycle Review | Pallav | 0.1 | Draft | All Section C |
| **Section D — Academic Delivery** | | | | |
| L2-D01 — Course & Course Offering Workflows | Pallav | 0.1 | Draft | Section B |
| L2-D02 — Faculty Academic Assignment Workflows | Pallav | 0.1 | Draft | L2-D01, Section E |
| L2-D03 — Timetable Workflows | Pallav | 0.1 | Draft | L2-D02 |
| L2-D04 — Teaching Session & Attendance Workflows | Pallav | 0.1 | Draft | L2-D03 |
| L2-D05 — Academic Monitoring Workflows | Pallav | 0.1 | Draft | L2-D04 |
| L2-D00 — Academic Delivery Review | Pallav | 0.1 | Draft | All Section D |
| **Section E — People & Workforce** | | | | |
| L2-E01 — Workforce & Engagement Workflows | Dhruv | 0.1 | Draft | Section A, Section B |
| L2-E02 — Position & Assignment Workflows | Dhruv | 0.1 | Draft | L2-E01 |
| L2-E03 — Faculty Workforce Workflows | Dhruv | 0.1 | Draft | L2-E02 |
| L2-E04 — HR Workflows | Dhruv | 0.1 | Draft | L2-E01 |
| L2-E05 — Leave Workflows | Dhruv | 0.1 | Draft | L2-E01 |
| L2-E06 — Payroll Workflows | Dhruv | 0.1 | Draft | L2-E04, L2-E05 |
| L2-E00 — People & Workforce Review | Dhruv | 0.1 | Draft | All Section E |
| **Section F — Finance & Examination** | | | | |
| L2-F01 — Finance Domain Workflows | Team 1 | 0.1 | Draft | Section A, B, C |
| L2-F02 — Examination Domain Workflows | Team 1 | 0.1 | Draft | Section C, D |
| L2-F03 — Team 1 Contract Boundary Review | Pallav | 0.1 | Draft | L2-F01, L2-F02 |
| L2-F00 — Finance & Examination Review | Team 1 | 0.1 | Draft | All Section F |
| **Section G — Campus Operations** | | | | |
| L2-G01 — Facilities Workflows | Pallav | 0.1 | Draft | Section B |
| L2-G02 — Lab Management Workflows | Pallav | 0.1 | Draft | L2-G01 |
| L2-G03 — Hostel Workflows | Pallav | 0.1 | Draft | Section C |
| L2-G04 — Transport Workflows | Pallav | 0.1 | Draft | Section C |
| L2-G05 — Library Workflows | Pallav | 0.1 | Draft | Section C |
| L2-G06 — Medical Workflows | Pallav | 0.1 | Draft | Section A |
| L2-G07 — Security Operations Workflows | Pallav | 0.1 | Draft | Section B |
| L2-G08 — Housekeeping Operations Workflows | Pallav | 0.1 | Draft | Section B |
| L2-G00 — Campus Operations Review | Pallav | 0.1 | Draft | All Section G |
| **Section H — Institutional Services & Intelligence** | | | | |
| L2-H01 — Document Domain Workflows | Pallav | 0.1 | Draft | Section A |
| L2-H02 — Notification Workflows | Pallav | 0.1 | Draft | Section A |
| L2-H03 — Directory Workflows | Pallav | 0.1 | Draft | Section A, B |
| L2-H04 — Placement Workflows | Pallav | 0.1 | Draft | Section C |
| L2-H05 — Research Workflows | Pallav | 0.1 | Draft | Section E |
| L2-H06 — Alumni Workflows | Pallav | 0.1 | Draft | Section C |
| L2-H07 — Dashboard Workflows | Pallav | 0.1 | Draft | All Domains |
| L2-H08 — Reporting Workflows | Pallav | 0.1 | Draft | All Domains |
| L2-H00 — Services & Intelligence Review | Pallav | 0.1 | Draft | All Section H |
