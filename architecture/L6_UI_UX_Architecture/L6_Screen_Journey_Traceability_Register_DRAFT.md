# SutraERP L6 — Screen / Journey Traceability Register — Draft

**Version:** 0.1.0  
**Status:** Draft  
**Owner:** Lead Architect  
**Architecture Level:** L6 — UI / UX Architecture  
**Last Updated:** 2026-09-16  
**Depends On:** `L6_Screen_and_Workspace_Inventory_DRAFT.md`; `L6_Persona_Journey_and_Navigation_Map_DRAFT.md`; `00_Governance/L6_Foundation_Controlled_Authorization_2026-09-16.md`; Locked L3 canonical corpus; L4 production corpus; Locked `L5_L4_to_L5_Traceability_Methodology.md`; approved/stable L5 client/security/contract architecture.  
**Downstream:** production L6 workspace/screen specifications; frontend implementation proceed gates.  

> This register records provenance. It does **not** transfer business ownership, create permissions, or authorize implementation merely because a row exists.

---

## 1. Purpose

Provide one controlled L6 traceability view that answers:

1. What upstream business scenarios materially justify this L6 workspace/surface?
2. Which L4 behavior specifications realize those scenarios?
3. Which L5 client/security/contract boundaries must every web realization preserve?
4. Which UX navigation profiles may consume the surface when upstream responsibility permits?
5. Is the mapping sufficient to begin a production L6 slice, or is additional decomposition/review still required?

This register is intentionally many-to-many. One L3/L4 scenario may contribute to several L6 surfaces, and one L6 workspace may compose many scenarios.

---

## 2. Authority Hierarchy

Where mappings disagree, use repository authority in this order:

1. Locked repository decisions and governance;
2. final L3 canonical scenario identity register;
3. authored/approved L3 scenario semantics;
4. L4 production specification realizing that L3 scenario;
5. canonical L4-to-L5 traceability methodology and applicable L5 specifications;
6. L6 foundation authorization;
7. this Draft register;
8. frontend implementation.

Candidate IDs from pre-final L3 registers are lineage only. Final `L3-X-NNN` identities govern.

---

## 3. Mapping Conventions

### 3.1 Scenario Span Notation

A notation such as:

```text
C:020–026
```

means:

```text
L3-C-020 … L3-C-026
```

and, because the L4 canonical template binds L4 identity to the parent L3 identity:

```text
L4-C-020 … L4-C-026
```

unless an individual row explicitly states otherwise.

Multiple spans are comma-separated.

### 3.2 L4 Identity Rule

The L4 template defines:

```text
Specification ID: L4-<ParentSection>-<ParentScenarioNumber>
Realizes Scenario: L3-<ParentSection>-<ParentScenarioNumber>
```

Therefore every final L3 ID in this register has a mechanically corresponding L4 specification identity.

### 3.3 Mandatory Web L5 Baseline

Every SutraERP web L6 surface inherits, at minimum, the applicable responsibilities of:

- `L5-G-01-001` — Client Platform Architecture Baseline;
- `L5-G-01-002` — Web Application Architecture;
- `L5-G-01-003` — Client Security, Session, State & Contract Consumption Architecture;
- `L5-C-01` — released synchronous contract boundary and applicable specializations;
- `L5-D-01` — authenticated technical context and applicable specializations;
- `L5-D-02` — authoritative authorization evaluation/enforcement;
- `L5-A-04` — Tenant/Institution protected-scope and isolation semantics.

This shared baseline is abbreviated below as **Web Client Baseline**. It is a document shorthand only, not a new architecture identifier.

### 3.4 Domain-Specific L5 Rule

The exact domain-specific technical contribution set for a production L6 slice shall be resolved from the applicable L4 BH-to-L5 relations under the Locked L5 traceability methodology.

This register does not duplicate the canonical L4→L5 relation store. Before a row becomes production-slice ready, the L6 slice review must record the materially applicable L5 specifications and evidence.

### 3.5 Trace States

| State | Meaning |
|---|---|
| `Scenario Anchored` | Exact final L3/L4 scenario population is resolved for the workspace family. |
| `Composite Anchored` | Shared L6 surface composes multiple upstream sources; no independent business ownership is implied. |
| `L5 Slice Resolution Pending` | Shared web-client L5 baseline is known, but exact domain-specific L5 contribution/evidence must be bound during slice authoring. |
| `Slice Ready` | Specific L6 slice has exact L3/L4/L5/actor/action trace and passes L6 review. This register does not automatically assign this state. |
| `Blocked` | An upstream ambiguity/conflict prevents safe L6 derivation. |

No row in version 0.1.0 is automatically `Slice Ready` solely by appearing here.

---

## 4. Inventory Correction Found During Traceability

The first workspace inventory counted 98 business workspace families by excluding L2 review artifacts. Full final-L3 traceability exposed one additional canonical business-outcome family:

```text
L3-G-001 … L3-G-015
Primary Owner: Campus Operations Governance
```

These scenarios are not a reason to convert `L2-G00 Campus Operations Review` into a business owner. They establish a separate L3-derived L6 workspace need for campus-operations governance/readiness/operational review.

Effective controlled count for subsequent L6 work:

| Item | Count |
|---|---:|
| L2-anchored business workspace families | 98 |
| Additional L3-derived Campus Operations Governance workspace family | 1 |
| Shared L6 surfaces | 6 |
| **Effective L6 inventory units** | **105** |

Provisional draft key used here:

```text
W-G00 — Campus Operations Governance
```

`W-G00` is a non-canonical L6 row key only. It does **not** redefine `L2-G00` or create an L2 owner.

---

## 5. Shared L6 Surface Traceability

| Key | Surface | Material L3/L4 basis | Primary UX profiles | L5 basis | State |
|---|---|---|---|---|---|
| `S-01` | Persona Home | Composite projections from the user's applicable workspace families; Admin additionally consumes `A:058` human-task context and `H:048–054` dashboard capability where applicable | Student; Faculty; Guardian; Institutional Admin; specialists | Web Client Baseline + applicable domain-specific L5 relations | Composite Anchored / L5 Slice Resolution Pending |
| `S-02` | Global Search / Command Discovery | `H:017–021` | All authenticated profiles where search is supported | Web Client Baseline + applicable H03/search L5 relations | Scenario Anchored / L5 Slice Resolution Pending |
| `S-03` | Notifications | `H:010–016` | All profiles where notifications are supported | Web Client Baseline + applicable H02/notification L5 relations | Scenario Anchored / L5 Slice Resolution Pending |
| `S-04` | Institution / Academic Context | Composite: `A:001–003,056–057`; `B:001–005,027–033`; applicable scope behavior in L5-G01/A04 | Multi-scope authenticated profiles | Web Client Baseline; A04/G01-003 materially mandatory | Composite Anchored / L5 Slice Resolution Pending |
| `S-05` | Account / Authentication Entry | `A:007–009,055` | Users requiring authentication | Web Client Baseline + applicable D01 authentication/session architecture | Scenario Anchored / L5 Slice Resolution Pending |
| `S-06` | Task / Work Entry | `A:058` plus owning-domain scenario context for each task | Institutional Admin and specialists; Faculty/Student only where an actual upstream task exists | Web Client Baseline + applicable workflow/owning-domain L5 relations | Scenario Anchored / L5 Slice Resolution Pending |

---

# 6. Business Workspace Traceability

## 6.1 Section A — Platform Foundation

| Key | Workspace | Final L3/L4 scenario span | UX profiles | State |
|---|---|---|---|---|
| `W-A01` | Tenant Administration | `A:001–003,056–057` | Platform / Tenant Administration | Scenario Anchored / L5 Slice Resolution Pending |
| `W-A02` | Person & Identity | `A:004–006,053–054` | Platform / Tenant Administration; contextual institutional specialists | Scenario Anchored / L5 Slice Resolution Pending |
| `W-A03` | Authentication & Sign-in | `A:007–009,055` | All applicable users via authentication entry | Scenario Anchored / L5 Slice Resolution Pending |
| `W-A04` | Access & Responsibility | `A:010–013,050–052` | Platform / Tenant Administration; authorized institutional governance actors | Scenario Anchored / L5 Slice Resolution Pending |
| `W-A05` | Workflow & Human Tasks | `A:014–017,058` | Platform / Tenant Administration; task-owning specialists | Scenario Anchored / L5 Slice Resolution Pending |
| `W-A06` | Event Governance | `A:018–021,059–060` | Platform / Tenant Administration | Scenario Anchored / L5 Slice Resolution Pending |
| `W-A07` | Audit & Evidence | `A:022–027` | Platform / Tenant Administration; authorized auditors/investigators | Scenario Anchored / L5 Slice Resolution Pending |
| `W-A08` | Capability / Module Registry | `A:028–032` | Platform / Tenant Administration | Scenario Anchored / L5 Slice Resolution Pending |
| `W-A09` | Master Data | `A:033–041` | Platform / Tenant Administration; authorized data-governance specialists | Scenario Anchored / L5 Slice Resolution Pending |
| `W-A10` | Operational Readiness | `A:042–049,061–062` | Platform / Tenant Administration; institutional governance | Scenario Anchored / L5 Slice Resolution Pending |

## 6.2 Section B — Institution & Structure

The final B ranges account for the approved `CAN-L3-B-021` split; candidate numbering after the split is not reused as final identity.

| Key | Workspace | Final L3/L4 scenario span | UX profiles | State |
|---|---|---|---|---|
| `W-B01` | Institution Administration | `B:001–005` | Institutional Administrator | Scenario Anchored / L5 Slice Resolution Pending |
| `W-B02` | Organisation Structure & Positions | `B:006–016` | Institutional Administrator; People/HR contextual use | Scenario Anchored / L5 Slice Resolution Pending |
| `W-B03` | Institutional Campus Registry | `B:017–022` | Institutional Administrator; Campus Operations | Scenario Anchored / L5 Slice Resolution Pending |
| `W-B04` | Curriculum Framework | `B:023–026` | Academic Operations; Institutional Administrator | Scenario Anchored / L5 Slice Resolution Pending |
| `W-B05` | Academic Calendar | `B:027–033` | Academic Operations; Institutional Administrator; read projections for Student/Faculty | Scenario Anchored / L5 Slice Resolution Pending |

## 6.3 Section C — Student Lifecycle

`L3-C-001` crosses the admission-to-active-student boundary and therefore may legitimately contribute to both Admissions and Student Workspace presentation. Traceability does not require artificial one-to-one ownership at L6.

| Key | Workspace | Final L3/L4 scenario span | UX profiles | State |
|---|---|---|---|---|
| `W-C01` | Admissions | `C:001–007` | Admissions & Student Operations; Institutional Admin | Scenario Anchored / L5 Slice Resolution Pending |
| `W-C02` | Student Workspace | `C:001,008–019` | Student Operations; Institutional Admin; Student self-service projections where supported | **Slice Ready — governed by `L6_Student_Workspace_Validation_and_Proceed_Gate_Decision_2026-09-17.md`; delivery increments tracked separately** |
| `W-C03` | Academic Registration | `C:020–026` | Student; Student Operations; Academic Operations | Scenario Anchored / L5 Slice Resolution Pending |
| `W-C04` | Academic Progression & Credits | `C:027–038,075` | Student; Student Operations; Academic Operations | Scenario Anchored / L5 Slice Resolution Pending |
| `W-C05` | Qualification & Exit | `C:039–050` | Student Operations; Institutional Admin; Student evidence/status where supported | Scenario Anchored / L5 Slice Resolution Pending |
| `W-C06` | Guardian Relationships | `C:051–061` | Guardian; Student Operations; Institutional Admin | Scenario Anchored / L5 Slice Resolution Pending |
| `W-C07` | Scholarship Eligibility | `C:062–071` | Student; Student Operations; Finance contextual handoff | Scenario Anchored / L5 Slice Resolution Pending |
| `W-C08` | Alumni Transition | `C:072–074` | Student/Alumni transition; Student Operations; Institutional Services | Scenario Anchored / L5 Slice Resolution Pending |

## 6.4 Section D — Academic Delivery

The final D ranges account for the D-027/D-028 merge and D-042 split recorded by the final L3 ID register.

| Key | Workspace | Final L3/L4 scenario span | UX profiles | State |
|---|---|---|---|---|
| `W-D01` | Course Offerings | `D:001–016` | Academic Operations; Faculty/Student projections where supported | Scenario Anchored / L5 Slice Resolution Pending |
| `W-D02` | Faculty Academic Assignment | `D:017–019` | Academic Operations; Faculty | Scenario Anchored / L5 Slice Resolution Pending |
| `W-D03` | Timetable | `D:020–022` | Student; Faculty; Academic Operations | Scenario Anchored / L5 Slice Resolution Pending |
| `W-D04` | Teaching Sessions & Attendance | `D:023–026` | Faculty; Student; Academic Operations | Scenario Anchored / L5 Slice Resolution Pending |
| `W-D05` | Academic Monitoring | `D:027–033` | Faculty; Academic Operations; Student contextual projection where supported | Scenario Anchored / L5 Slice Resolution Pending |
| `W-D06` | Flexible Learning / Electives / MOOCs | `D:034–050` | Student; Faculty; Academic Operations | Scenario Anchored / L5 Slice Resolution Pending |
| `W-D07` | Internal Assessment | `D:051–057` | Student; Faculty; Academic Operations | Scenario Anchored / L5 Slice Resolution Pending |

## 6.5 Section E — People & Workforce

| Key | Workspace | Final L3/L4 scenario span | UX profiles | State |
|---|---|---|---|---|
| `W-E01` | Workforce Engagement | `E:001–010` | People / HR Operations; Institutional Admin | Scenario Anchored / L5 Slice Resolution Pending |
| `W-E02` | Position & Assignment | `E:011–025,071–073` | People / HR Operations; Institutional Admin | Scenario Anchored / L5 Slice Resolution Pending |
| `W-E03` | Faculty Workforce | `E:026–032` | People / HR Operations; Faculty; Academic Operations | Scenario Anchored / L5 Slice Resolution Pending |
| `W-E04` | HR Services & Cases | `E:033–043` | People / HR Operations; employee self-service where supported | Scenario Anchored / L5 Slice Resolution Pending |
| `W-E05` | Leave Management | `E:044–052` | People / HR Operations; employee/faculty self-service where supported | Scenario Anchored / L5 Slice Resolution Pending |
| `W-E06` | Payroll | `E:053–057,074` | People / HR Operations; Finance contextual handoff; employee read projections where supported | Scenario Anchored / L5 Slice Resolution Pending |
| `W-E07` | Employee Profile | `E:058–070,075` | People / HR Operations; employee/faculty self-service where supported | Scenario Anchored / L5 Slice Resolution Pending |

## 6.6 Section F — Finance

Section F retained direct final positional alignment.

| Key | Workspace | Final L3/L4 scenario span | UX profiles | State |
|---|---|---|---|---|
| `W-F00` | Finance Control & Configuration | `F:084–085` | Finance Operations | Scenario Anchored / L5 Slice Resolution Pending |
| `W-F01` | Student Fees & Charges | `F:001–006` | Finance Operations; Student/Guardian projections where responsibility permits | Scenario Anchored / L5 Slice Resolution Pending |
| `W-F02` | Payments & Receipts | `F:007–013` | Finance Operations; Student/Guardian payment participation where supported | Scenario Anchored / L5 Slice Resolution Pending |
| `W-F03` | Refunds | `F:014–019` | Finance Operations; requesting participant where supported | Scenario Anchored / L5 Slice Resolution Pending |
| `W-F04` | Financial Adjustments | `F:020–024` | Finance Operations | Scenario Anchored / L5 Slice Resolution Pending |
| `W-F05` | General Ledger | `F:025–032` | Finance Operations | Scenario Anchored / L5 Slice Resolution Pending |
| `W-F06` | Budgets & Cost Centres | `F:033–038` | Finance Operations; authorized budget stakeholders | Scenario Anchored / L5 Slice Resolution Pending |
| `W-F07` | Accounts Receivable | `F:039–043` | Finance Operations | Scenario Anchored / L5 Slice Resolution Pending |
| `W-F08` | Accounts Payable | `F:044–050` | Finance Operations; procurement contextual handoff | Scenario Anchored / L5 Slice Resolution Pending |
| `W-F09` | Financial Periods & Closing | `F:051–055` | Finance Operations | Scenario Anchored / L5 Slice Resolution Pending |
| `W-F10` | Treasury & Banking | `F:056–061` | Finance Operations | Scenario Anchored / L5 Slice Resolution Pending |
| `W-F11` | Financial Aid | `F:062–066` | Finance Operations; Student contextual projection where supported | Scenario Anchored / L5 Slice Resolution Pending |
| `W-F12` | Asset Accounting | `F:067–072` | Finance Operations; Resource/Asset contextual handoff | Scenario Anchored / L5 Slice Resolution Pending |
| `W-F13` | Financial Reports & Analytics | `F:073–079` | Finance Operations; Institutional Admin/Insights where authorized | Scenario Anchored / L5 Slice Resolution Pending |
| `W-F14` | Procurement Finance | `F:080–083` | Finance Operations; Procurement/Resources contextual handoff | Scenario Anchored / L5 Slice Resolution Pending |

## 6.7 Section G — Campus Operations

The final G ranges below apply the two final-ID absorptions: G-002 into the G-001 survivor, and G-164 into the G-163 survivor. The L3-derived governance workspace is retained separately from G01–G10.

| Key | Workspace | Final L3/L4 scenario span | UX profiles | State |
|---|---|---|---|---|
| `W-G00` | Campus Operations Governance | `G:001–015` | Institutional Administrator; Campus Operations; Insights/Compliance contextual use | Scenario Anchored / L5 Slice Resolution Pending |
| `W-G01` | Facilities | `G:016–049` | Campus Operations; authorized Student/Faculty service-request projections where supported | Scenario Anchored / L5 Slice Resolution Pending |
| `W-G02` | Laboratories | `G:050–077` | Campus Operations; Faculty/Student laboratory context where supported | Scenario Anchored / L5 Slice Resolution Pending |
| `W-G03` | Hostel | `G:078–092` | Campus Operations; Student service context where supported | Scenario Anchored / L5 Slice Resolution Pending |
| `W-G04` | Transport | `G:093–123` | Campus Operations; Student/Employee service context where supported | Scenario Anchored / L5 Slice Resolution Pending |
| `W-G05` | Library | `G:124–154` | Campus Operations; Student/Faculty/Employee library services where supported | Scenario Anchored / L5 Slice Resolution Pending |
| `W-G06` | Health / Medical Services | `G:155–185` | Campus Operations; Student/Employee health-service context where supported | Scenario Anchored / L5 Slice Resolution Pending |
| `W-G07` | Security Operations | `G:186–215` | Campus Operations; contextual institutional users | Scenario Anchored / L5 Slice Resolution Pending |
| `W-G08` | Housekeeping & Service Governance | `G:216–251` | Campus Operations; contextual service requesters | Scenario Anchored / L5 Slice Resolution Pending |
| `W-G09` | Teacher Guardian Scheme | `G:252–277` | Faculty/Teacher Guardian; Student; Campus Operations | Scenario Anchored / L5 Slice Resolution Pending |
| `W-G10` | Infrastructure Management | `G:278–320` | Campus Operations; Institutional Administrator | Scenario Anchored / L5 Slice Resolution Pending |

## 6.8 Section H — Institutional Services & Intelligence

Section H retained direct final positional alignment.

| Key | Workspace | Final L3/L4 scenario span | UX profiles | State |
|---|---|---|---|---|
| `W-H01` | APAAR & DigiLocker | `H:001–009` | Institutional Services; Student where applicable | Scenario Anchored / L5 Slice Resolution Pending |
| `W-H02` | Notifications | `H:010–016` | Institutional Services; shared shell users | Scenario Anchored / L5 Slice Resolution Pending |
| `W-H03` | Enterprise Directory & Search | `H:017–021` | Institutional Services; shared shell users | Scenario Anchored / L5 Slice Resolution Pending |
| `W-H04` | Placement | `H:022–028` | Institutional Services; Student; relevant staff | Scenario Anchored / L5 Slice Resolution Pending |
| `W-H05` | Research | `H:029–038` | Institutional Services; Faculty/Research actors | Scenario Anchored / L5 Slice Resolution Pending |
| `W-H06` | Alumni Engagement | `H:039–047` | Institutional Services; Alumni-transition participants | Scenario Anchored / L5 Slice Resolution Pending |
| `W-H07` | Enterprise Dashboards | `H:048–054` | Institutional Administrator; specialist intelligence users | Scenario Anchored / L5 Slice Resolution Pending |
| `W-H08` | Statutory / Regulatory Reporting | `H:055–060` | Institutional Services / Intelligence; authorized evidence owners | Scenario Anchored / L5 Slice Resolution Pending |
| `W-H09` | Accreditation & Compliance | `H:061–071` | Institutional Services / Intelligence; institutional governance | Scenario Anchored / L5 Slice Resolution Pending |
| `W-H10` | External Education Integrations | `H:072–081` | Institutional Services; Student/Academic contextual interactions where supported | Scenario Anchored / L5 Slice Resolution Pending |

## 6.9 Section I — Examination

The final I ranges below apply the approved merges into final `I-072` and `I-074`; all ranges use final IDs, not pre-merge candidate numbers.

| Key | Workspace | Final L3/L4 scenario span | UX profiles | State |
|---|---|---|---|---|
| `W-I01` | Examination Policy & Configuration | `I:001–003` | Examination Operations | Scenario Anchored / L5 Slice Resolution Pending |
| `W-I02` | Examination Sessions | `I:004–006` | Examination Operations | Scenario Anchored / L5 Slice Resolution Pending |
| `W-I03` | Examination Planning & Scheduling | `I:007–016` | Examination Operations; Faculty/Student timetable projection where supported | Scenario Anchored / L5 Slice Resolution Pending |
| `W-I04` | Examination Registration | `I:017–036` | Examination Operations; Student/Candidate | Scenario Anchored / L5 Slice Resolution Pending |
| `W-I05` | Hall Tickets | `I:037–042` | Examination Operations; Student/Candidate | Scenario Anchored / L5 Slice Resolution Pending |
| `W-I06` | Practical & Viva Conduct | `I:043–054` | Examination Operations; Faculty/examiners; Candidate context | Scenario Anchored / L5 Slice Resolution Pending |
| `W-I07` | Theory Examination Conduct | `I:055–070` | Examination Operations; Faculty/invigilators; Candidate context | Scenario Anchored / L5 Slice Resolution Pending |
| `W-I08` | Marks & Evaluation | `I:071–082` | Examination Operations; authorized evaluators | Scenario Anchored / L5 Slice Resolution Pending |
| `W-I09` | Result Processing | `I:083–100` | Examination Operations | Scenario Anchored / L5 Slice Resolution Pending |
| `W-I10` | Result Publication | `I:101–117` | Examination Operations; Student/Candidate read outcome | Scenario Anchored / L5 Slice Resolution Pending |
| `W-I11` | Revaluation & Rechecking | `I:118–132` | Examination Operations; Student/Candidate request context | Scenario Anchored / L5 Slice Resolution Pending |
| `W-I12` | Supplementary / Backlog / Improvement Exams | `I:133–144` | Examination Operations; Student/Candidate | Scenario Anchored / L5 Slice Resolution Pending |
| `W-I13` | Academic Completion & CGPA Evidence | `I:145–161` | Examination Operations; Student/Student Operations contextual evidence | Scenario Anchored / L5 Slice Resolution Pending |
| `W-I14` | Malpractice & Examination Committee | `I:162–171` | Examination Operations; committee/authorized review actors | Scenario Anchored / L5 Slice Resolution Pending |
| `W-I15` | Examination Exceptions & Contingency | `I:172–186` | Examination Operations; institutional governance | Scenario Anchored / L5 Slice Resolution Pending |
| `W-I16` | Examination Closure & Archive | `I:187–192` | Examination Operations; authorized historical evidence users | Scenario Anchored / L5 Slice Resolution Pending |

## 6.10 Section J — Enterprise Resource Management

The final J ranges account for disposal-method variants absorbed into the final J-097 survivor and later candidates remapped to final J-099…104.

| Key | Workspace | Final L3/L4 scenario span | UX profiles | State |
|---|---|---|---|---|
| `W-J01` | Procurement Planning & Sourcing | `J:001–019` | Procurement / Resource Operations | Scenario Anchored / L5 Slice Resolution Pending |
| `W-J02` | Vendors & Suppliers | `J:020–037` | Procurement / Resource Operations | Scenario Anchored / L5 Slice Resolution Pending |
| `W-J03` | Purchase Requisitions & Approvals | `J:038–042` | Procurement / Resource Operations; authorized requesters/approvers | Scenario Anchored / L5 Slice Resolution Pending |
| `W-J04` | Purchase Orders | `J:043–048` | Procurement / Resource Operations | Scenario Anchored / L5 Slice Resolution Pending |
| `W-J05` | Goods Receipt & Inspection | `J:049–050` | Procurement / Resource Operations | Scenario Anchored / L5 Slice Resolution Pending |
| `W-J06` | Inventory & Warehouse | `J:051–063` | Procurement / Resource Operations | Scenario Anchored / L5 Slice Resolution Pending |
| `W-J07` | Stock Operations & Distribution | `J:064–073` | Procurement / Resource Operations; contextual requesting domains | Scenario Anchored / L5 Slice Resolution Pending |
| `W-J08` | Asset Lifecycle | `J:074–086` | Procurement / Resource Operations; asset custodians where supported | Scenario Anchored / L5 Slice Resolution Pending |
| `W-J09` | Physical Verification & Reconciliation | `J:087–093` | Procurement / Resource Operations; auditors/verifiers | Scenario Anchored / L5 Slice Resolution Pending |
| `W-J10` | Disposal / Scrap / Write-Off | `J:094–104` | Procurement / Resource Operations; Finance contextual handoff | Scenario Anchored / L5 Slice Resolution Pending |

---

# 7. Coverage Reconciliation

The exact final scenario populations represented by the business-workspace rows reconcile to the final L3 corpus when overlap is treated correctly:

- A: all 62 final scenarios represented;
- B: all 33 represented;
- C: all 75 represented; `C-001` intentionally contributes to both Admissions and Student Workspace;
- D: all 57 represented;
- E: all 75 represented;
- F: all 85 represented;
- G: all 320 represented only after adding `W-G00`;
- H: all 81 represented;
- I: all 192 represented;
- J: all 104 represented.

Therefore all **1,084 final L3 scenarios have a defensible L6 workspace-family or shared/composite presentation destination** at this planning level.

This does not mean every scenario requires its own screen.

---

# 8. L5 Binding Gate for Production Slices

Before any workspace/surface receives `Slice Ready`, the corresponding L6 specification shall record:

1. exact L3 IDs used by that slice;
2. exact L4 specification and material BH IDs;
3. exact L5 specifications contributing to those BHs;
4. L5 evidence section / AD / CR / ADR where applicable;
5. supported actor/responsibility evidence;
6. authoritative facts presented;
7. actions exposed and their authoritative outcome owners;
8. context/scope constraints;
9. loading/empty/pending/rejection/indeterminate/failure behavior;
10. accessibility and responsive completion conditions.

The Locked L5 traceability methodology remains authoritative for L4 BH→L5 relation semantics. L6 shall not create a parallel competing L5 mapping store.

---

# 9. Recommended First Production Trace Slices

The first production L6 specifications should be chosen from high-frequency, high-reuse journeys already represented in the approved shell direction, while still completing the exact L5 binding gate above.

Candidate sequence for controlled authoring:

```text
1. Shared authenticated application shell + context/search/notification boundaries
2. Student Workspace overview/read-only orientation
3. Timetable — Student + Faculty projections
4. Teaching Session & Attendance — Faculty operational journey
5. Academic Registration — Student journey
6. Internal Assessment — Faculty + Student journey
7. Student Fees & Charges / Payments — Student read/pay journey
8. Administrator Admissions attention/work journey
9. Examination Registration / Hall Ticket / Result Publication — Student journey
10. Specialist institutional workspaces thereafter by dependency and frequency
```

This is a recommended authoring order, not a new business dependency map and not implementation authorization by itself.

---

# 10. Integrity Findings and Resolutions

## TR-L6-001 — Campus Governance Coverage Gap

**Finding:** Initial 104-unit inventory omitted the L3-owned Campus Operations Governance scenario family because it excluded review-only L2 documents mechanically.  
**Impact:** `L3-G-001…015` had no explicit workspace-family destination.  
**Resolution:** Introduce provisional `W-G00 Campus Operations Governance` as an L3-derived L6 workspace family. Do not redefine `L2-G00`. Effective inventory count becomes 105 units.  
**Status:** Resolved in this register; source inventory/README/index require synchronization.

## TR-L6-002 — Candidate-ID Drift Risk

**Finding:** Sections B, D, G, I and J contain split/merge/variant transformations; candidate numbers cannot be used as final IDs blindly.  
**Resolution:** All mappings in this register use final L3 IDs after applying the Final Canonical Scenario ID Register.  
**Status:** Resolved.

## TR-L6-003 — Domain-Specific L5 Evidence Not Yet Bound Per Slice

**Finding:** Workspace-family ranges are now exact at L3/L4 identity level, but exact BH→L5 contribution sets still need slice-level binding.  
**Impact:** No business workspace should yet be treated as production-screen authorized solely from this register.  
**Resolution:** Preserve `L5 Slice Resolution Pending` until each production L6 slice records exact L5 evidence and passes review.  
**Status:** Open by design; non-blocking for planning, blocking for uncontrolled production screen authoring.

---

# 11. Proceed Gate

**Planning traceability verdict:** PASS WITH CONTROLLED FOLLOW-UP.

Established:

- all 1,084 final L3 scenarios reconciled to L6 workspace-family/shared presentation coverage;
- deterministic L3→L4 identity linkage established;
- mandatory web-client L5 baseline established;
- actor/profile presentation scope identified without creating authorization roles;
- one inventory gap detected and explicitly resolved at L6 planning level.

Still required before production slice authorization:

- exact material BH selection for the slice;
- exact domain-specific L5 relation/evidence binding;
- L6 workspace/screen decomposition;
- primary/alternate/error-state design;
- accessibility/responsive acceptance;
- independent L6 validation.

---

**Draft verdict:** complete repository-wide L3/L4 workspace-family traceability baseline established; exact per-slice L5 evidence binding remains the next controlled authoring gate.


---

## 8. Delivery Slice Linkage

This register remains the workspace-family/upstream traceability view. It does not expand each workspace row into delivery experience slices and does not duplicate canonical BH→L5 relations.

Where an approved L6 package is decomposed for implementation, `L6_Experience_Slice_Register_DRAFT.md` may point back to this workspace row, the controlling L6 specification/proceed-gate decision, the canonical L4→L5 trace evidence, the applicable review pack and implementation/test evidence.

A delivery slice inherits only the approved boundary it actually uses; it does not enlarge the governing package.


---

## 12. Workflow Realization Expansion Linkage

The 2026-09-19 controlled L6 reopening adds a new realization question without changing this register's authority:

> For every canonical L3 scenario already reconciled to workspace-family/shared-presentation coverage, how is that scenario actually interacted with or observed in the frontend?

That question is owned by `L6_Workflow_Realization_Register_DRAFT.md`.

This traceability register continues to own workspace-family/upstream provenance. The workflow-realization register shall reference this evidence rather than duplicate or replace it.

Required expansion relationship:

```text
canonical L3 scenario
→ this register: upstream/workspace-family traceability
→ workflow realization register: primary UI disposition + user-facing realization
→ workspace/search/task/handoff/history destination
→ mock interaction or observable outcome
→ review evidence
```

A scenario's presence in this register does not mean it is interactively implemented. The expansion completion gate requires an explicit realization disposition for all 1,084 canonical scenarios.

System/background scenarios must not be converted into user actions merely to satisfy coverage.
