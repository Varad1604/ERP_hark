# L5 Solution Architecture

**Two L5 source documents are present in this folder.** Corrected 2026-08-06 under
integrity finding INT-201; this README previously stated that none existed.

| File | Created | Extracted from | Under finding |
|---|---|---|---|
| `L5-00_Technology_Stack_Baseline.md` | 2026-08-04 | `architecture/L0_Business_Vision/SutraERP_Architecture_Workflow_Context_Baseline.md` §3 | INT-002 |
| `L5-01_Infrastructure_Persistence_Baseline.md` | 2026-08-05 | `architecture/L2_Domain_Workflow_Architecture/section-b/L2-B03_Campus_Workflows.md` Part 14 | INT-008 |

Both were created to remove level violations from upstream documents — a technology stack
held in L0, and persistence design held in L2. Neither has been reviewed as L5
architecture and **neither traces to an approved L4 System Behavior Specification,
because no L4 document exists.** Both are `Draft`. Treat them as recorded extractions,
not as an approved L5 baseline.

Future documents must trace to approved L4 System Behavior Specification and must not redefine business rules.

Use `templates/L5_SolutionArchitecture_Template.md` when an approved L5 artifact is authorized.
