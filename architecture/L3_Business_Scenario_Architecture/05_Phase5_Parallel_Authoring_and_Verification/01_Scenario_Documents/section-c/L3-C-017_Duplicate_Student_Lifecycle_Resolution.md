# Duplicate Student Lifecycle Resolution

**Scenario ID:** `L3-C-017`  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Student Lifecycle  
**Participating Domains:** A, C  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** L2-A02 — Person & Identity Workflows; L2-C02 — Student Lifecycle & Activation Workflows  
**Referenced By:** None yet

> **Phase 5 authoring note:** This is a complete L3 Draft. It is not Architecture Approved or Locked. The frozen C017 register identity and classification are preserved exactly. fileciteturn95file0L2-L2

---

## 1.1 Canonical Scenario Identity

| Attribute | Value |
|---|---|
| Final L3 ID | `L3-C-017` |
| Canonical Scenario Title | **Duplicate Student Lifecycle Resolution** |
| Primary Owner | **Student Lifecycle** |
| Participating Domains | **A, C** |
| Scenario Type | **Critical** |
| Priority | **P1** |
| Derivation | **KEEP** |
| Construction Status in Stage 3 | Not Authored |

The Stage 3 Section C register identifies C017 as `Duplicate Student Lifecycle Resolution`, owned by Student Lifecycle, participating domains A and C, with Type Critical, Priority P1, and Derivation KEEP. fileciteturn95file0L2-L2

## 1.2 Applicable Locked L2 Sources

### L2-C02 — Student Lifecycle & Activation

The authoritative C02 workflow defines `WF-C02-16 — Student Merge & Duplicate Resolution Workflow`, including duplicate validation, canonical survivor selection, mergeable business data, approval, reference migration, duplicate archival, `StudentMerged`, audit requirements, postconditions and terminal outcomes. fileciteturn102file0L2-L2 fileciteturn103file0L2-L2 fileciteturn104file0L2-L2

C02 also establishes Student Lifecycle as the canonical owner of the Student Aggregate and prohibits duplicate active Student identities. fileciteturn97file0L2-L2

### L2-A02 — Person & Identity

The authoritative A02 architecture establishes Person as the canonical identity aggregate, with exactly one canonical Person per human within a Tenant. Business domains consume Person references and may not create independent Person entities or directly modify the Person aggregate. fileciteturn105file0L2-L2 fileciteturn106file0L2-L2

A02 also defines Person duplicate detection and Person merge as canonical identity capabilities, with `PersonMerged` as an A02-owned event. fileciteturn108file0L2-L2 fileciteturn109file0L2-L2

---

## 2. Scenario Purpose

This scenario governs the institutional resolution of duplicate Student records when more than one Student record represents the same underlying student identity or otherwise satisfies the authoritative duplicate-resolution criteria.

The purpose is to establish one canonical Student record, preserve the historical lineage of the records being resolved, reconcile the permitted Student business information, protect downstream business references, and maintain canonical ownership across Student Lifecycle and Person & Identity.

The scenario is a **resolution of duplicate Student lifecycle records**, not creation of a new student identity.

---

## 3. Business Outcome

> **One canonical Student record remains authoritative and operational, duplicate Student records are removed from operational use through governed archival, the permitted business information and historical lineage are reconciled without loss of institutional history, downstream Student references can be aligned to the canonical Student, and Person identity ownership remains governed by the canonical Person & Identity domain.**

The canonical Student survivor retains its Student identity and lifecycle ownership.

No new Student identity is created merely to resolve the duplicate.

---

## 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when duplicate Student records have been identified as candidates for institutional duplicate resolution and the governed Student Merge & Duplicate Resolution process is initiated.

The locked C02 workflow requires that both Student records exist, belong to the same Tenant, duplicate suspicion has been established, merge is permitted by institutional policy, required approvals are configured, and the caller is authorised. fileciteturn103file0L2-L2

## 4.2 End Boundary

The scenario ends when:

1. one Student has been established as the canonical survivor;
2. permitted Student business information has been reconciled;
3. approved business references have been migrated to the canonical Student;
4. the duplicate Student record has been archived;
5. historical lineage remains preserved;
6. the merge is fully auditable;
7. `StudentMerged` has been published by Student Lifecycle; and
8. required downstream Student references can reconcile to the canonical Student through approved architecture boundaries.

The Main Business Flow stops at this completed Student Lifecycle outcome.

## 4.3 In Scope

- Duplicate Student candidate validation.
- Duplicate investigation at the Student business level.
- Canonical survivor selection.
- Merge approval.
- Reconciliation of permitted Student business information.
- Preservation of enrollment and lifecycle history.
- Migration of approved Student business references.
- Archival of the duplicate Student record.
- Audit of the merge decision and resulting resolution.
- Publication of the canonical `StudentMerged` event.
- Cross-domain coordination with the canonical Person & Identity domain where Person identity is implicated.

The locked C02 workflow explicitly includes duplicate identification, investigation, approval, survivor selection, attribute reconciliation, reference migration, duplicate archival, audit history and cross-domain synchronization. fileciteturn102file0L2-L2

## 4.4 Out of Scope

- Applicant merge.
- Admission correction.
- Financial ledger reconciliation.
- Examination-result reconciliation.
- Document regeneration.
- External identity-registry updates.
- Creation of a replacement Student.
- Creation of a second Person identity.
- Technical data migration design.
- Database reconciliation.
- API or integration implementation.
- UI design.
- technical access control/IAM.
- Infrastructure or deployment behaviour.

The C02 source explicitly excludes applicant merge, admission correction, examination result reconciliation, financial ledger reconciliation, document regeneration and external identity registry updates from WF-C02-16. fileciteturn102file0L2-L2

## 4.5 Related Scenarios

| Relationship | Related Scenario | Reason |
|---|---|---|
| Related | **L3-C-016 — Student Identity Correction** | C016 corrects an existing Student identity; C017 resolves duplicate Student lifecycle records. |
| Related | **L3-C-001 — Approved Candidate to Active Student** | Student records entering the Student Lifecycle are downstream of admission-to-student establishment. |

No additional relationship is asserted where a final canonical scenario ID was not verified.

## 4.6 Boundary with Person & Identity

L2-A02 owns the canonical Person and Identity aggregates and states that business domains do not create independent Person entities or directly modify the Person aggregate. fileciteturn106file0L2-L2

Therefore:

- C017 owns resolution of duplicate **Student** lifecycle records.
- A02 remains owner of **Person/Identity**.
- Student merge does not automatically imply a new Person merge decision.
- If duplicate Student records are associated with duplicate Person records, the Person-side resolution must follow the authoritative A02 Person merge architecture.

This preserves the A/C ownership boundary.

---

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| **C — Student Lifecycle** | Canonical owner of duplicate Student lifecycle resolution, canonical Student survivor selection, Student business-data reconciliation, Student reference migration, duplicate Student archival, and the `StudentMerged` outcome. |
| **A — Person & Identity** | Canonical owner of Person and Identity concepts; participates where duplicate Student records involve canonical Person/Identity relationships and any Person-side identity resolution is required. |

## 5.1 Student Lifecycle Ownership

L2-C02 identifies Student Lifecycle as the canonical owner of the Student Aggregate and states that Student ownership remains within the domain. fileciteturn97file0L2-L2

The C02 merge workflow explicitly states that Student Lifecycle remains the authoritative owner of duplicate resolution and merge operations. fileciteturn102file0L2-L2

## 5.2 Person & Identity Ownership

L2-A02 states that Person, Identity and User Account are owned by A02 and that business domains may only reference Person identifiers through published contracts and events. fileciteturn105file0L2-L2 fileciteturn106file0L2-L2

A02 therefore remains authoritative for any Person-side identity resolution.

## 5.3 Ownership Constraint

Participation by A does not transfer Student ownership to A.

Participation by C does not transfer Person/Identity ownership to C.

No cross-domain ownership is introduced by this scenario.

---

## 6. Business Actors, Roles and Responsibilities

The locked C02 merge workflow explicitly identifies the **Registrar** as the business actor responsible for reviewing and approving merge requests. fileciteturn102file0L2-L2

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| **Registrar** | Reviews and approves Student merge requests. | Reviews and approves the governed Student merge decision. |
| **Student Lifecycle** | Performs canonical Student duplicate resolution. | Owns the Student merge outcome and resulting Student lifecycle record. |
| **Person & Identity** | Governs Person/Identity concepts when implicated by the Student resolution. | Retains canonical Person/Identity ownership. |

The source does not define a C017-specific A02 business actor responsible for a Person-side merge decision. No additional role is invented.

---

## 7. Preconditions and Entry Criteria

The following business conditions must be true before C017 begins:

1. Both Student records exist.
2. Both Student records belong to the same Tenant.
3. Duplicate suspicion has been established.
4. Institutional policy permits the Student merge.
5. Required approvals are configured.
6. The initiating participant is authenticated.
7. The initiating participant is authorised.
8. Neither Student is participating in conflicting business operations that prevent resolution.

These are explicit preconditions of `WF-C02-16`. fileciteturn103file0L2-L2

The canonical Student model also prohibits duplicate active Student identities and requires Student ownership to remain within Student Lifecycle. fileciteturn97file0L2-L2

---

## 8. Trigger

**Duplicate Student records are identified and a governed Student Merge & Duplicate Resolution request is initiated.**

The locked C02 workflow identifies the Registrar Portal, Administrative Portal, approved duplicate-detection process, data-migration process and Workflow Engine as initiation contexts. At L3 these are treated only as business initiation contexts; their implementation mechanics are outside this scenario. fileciteturn103file0L2-L2

---

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Confirm that the identified Student records are valid duplicate candidates under institutional policy. | Student Lifecycle | Candidate duplicate set is established. |
| 2 | Confirm that the candidate records satisfy the institutional conditions for merge. | Student Lifecycle | Merge remains eligible for business decision. |
| 3 | Establish the Student that will remain the canonical survivor. | Student Lifecycle / Registrar | One canonical Student is selected. |
| 4 | Review and approve the proposed Student merge. | Registrar | Merge decision is approved. |
| 5 | Reconcile permitted Student business information in accordance with institutional policy. | Student Lifecycle | Canonical Student contains the authoritative reconciled Student information. |
| 6 | Preserve enrollment history, lifecycle history and audit history associated with the duplicate records. | Student Lifecycle | Historical lineage remains traceable. |
| 7 | Migrate approved Student business references to the canonical survivor. | Student Lifecycle | Dependent Student references can resolve to the canonical Student. |
| 8 | Archive the merged duplicate Student record so it no longer remains operational. | Student Lifecycle | Duplicate Student is no longer an active operational identity. |
| 9 | Publish the canonical `StudentMerged` business event. | Student Lifecycle | Dependent domains receive the authoritative Student merge outcome. |
| 10 | Reconcile downstream Student references through approved business collaboration mechanisms. | Participating dependent domains | Enterprise Student references align with the canonical survivor. |
| 11 | Complete the scenario with one canonical Student remaining authoritative. | Student Lifecycle | Duplicate Student Lifecycle Resolution is complete. |

This flow follows the locked C02 sequence: validate candidates → validate policy → select canonical Student → validate conflicts → approval → reconcile attributes → migrate references → archive duplicate → audit → publish `StudentMerged` → synchronize downstream domains. fileciteturn103file0L2-L2

---

## 10. Alternate Business Flows

## 10.1 Multiple Duplicate Records

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Multiple duplicates | More than one duplicate Student record is identified for the same canonical Student lifecycle identity. | Select one canonical survivor and resolve one or more merged Student records under the same governed merge process. | One canonical Student remains; all duplicate Student records are archived after successful merge. | Student Lifecycle |

The C02 merge strategy explicitly supports one canonical survivor and one or more merged Student records. fileciteturn103file0L2-L2

## 10.2 Person Identity Requires Separate Resolution

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Person-side identity implication | Duplicate Student records involve Person identities whose canonical relationship requires separate identity resolution. | Preserve Student Lifecycle ownership of the Student merge while applying the authoritative A02 Person/Identity process for any Person-side merge or identity resolution. | Student and Person ownership boundaries remain intact. | Student Lifecycle + Person & Identity |

A02 explicitly owns duplicate detection and Person merge and publishes `PersonMerged`. fileciteturn108file0L2-L2 fileciteturn109file0L2-L2

The exact sequencing between Student merge and Person merge is not asserted because the retrieved authoritative sources do not define that cross-domain sequence explicitly. The gap is recorded in Section 17.

---

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Duplicate candidates cannot be validated | Merge cannot proceed. | Terminate the merge without changing business state. | Student Lifecycle |
| Students belong to different Tenants | Merge is not permitted. | Terminate the merge. | Student Lifecycle |
| Conflicting business constraints cannot be resolved | Canonical merge cannot safely be completed. | Terminate the merge without changing business state. | Student Lifecycle |
| Required approval is rejected | Merge does not complete. | Preserve existing Student records and terminate the merge request. | Student Lifecycle / Registrar |
| Authorisation is insufficient | Merge cannot proceed. | Terminate the merge request. | Student Lifecycle |
| Required configuration/policy validation fails | Merge cannot proceed. | Terminate the merge request until the governing condition is resolved. | Student Lifecycle |
| Referential integrity cannot be preserved | Merge cannot complete safely. | Terminate the merge without changing business state. | Student Lifecycle |
| Person/Identity ownership conflict is identified | Student merge cannot independently resolve the Person identity. | Preserve A02 ownership and defer the Person-side resolution to the authoritative A02 process. | Student Lifecycle + Person & Identity |

The C02 workflow explicitly states that these failure conditions terminate the workflow and that failures shall not modify business state. fileciteturn103file0L2-L2

No technical retry, rollback, queue, timeout, server failure or infrastructure recovery is defined.

---

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Student ownership remains within Student Lifecycle. | C | The canonical Student merge decision and outcome remain C-owned. | L2-C02 §6.6 / WF-C02-16. fileciteturn97file0L2-L2 |
| Duplicate active Student identities are prohibited. | C | Duplicate operational Student identities cannot remain active. | L2-C02 §6.6. fileciteturn97file0L2-L2 |
| Both Student records must exist. | C | A merge cannot be performed against a missing Student record. | WF-C02-16.5. fileciteturn103file0L2-L2 |
| Both Students must belong to the same Tenant. | C / Platform policy | Cross-Tenant Student merge is prohibited. | WF-C02-16.5. fileciteturn103file0L2-L2 |
| Duplicate suspicion must be established. | C | Unsubstantiated Student records cannot be merged. | WF-C02-16.5. fileciteturn103file0L2-L2 |
| Merge must be permitted by institutional policy. | C | Policy determines whether the merge may proceed. | WF-C02-16.5. fileciteturn103file0L2-L2 |
| One canonical survivor must be selected. | C | The merge does not create a new Student identity. | WF-C02-16.7. fileciteturn103file0L2-L2 |
| Canonical Student retains Student ID and Student Number. | C | Primary Student identity is preserved. | WF-C02-16.7. fileciteturn103file0L2-L2 |
| Permitted business data may be reconciled according to institutional policy. | C | Reconciliation is governed rather than arbitrary. | WF-C02-16.8. fileciteturn103file0L2-L2 |
| Enrollment, lifecycle and audit history are preserved. | C | Historical institutional lineage is not discarded. | WF-C02-16.8. fileciteturn103file0L2-L2 |
| Primary identifiers are never merged into a new identity. | C | The outcome remains a canonical survivor, not a newly generated Student. | WF-C02-16.8. fileciteturn103file0L2-L2 |
| Downstream domains update references through approved Service Contracts or Domain Events only. | C / participating domains | Downstream participation does not transfer Student ownership. | WF-C02-16.12. fileciteturn103file0L2-L2 |
| Person has exactly one canonical identity per Tenant. | A | Student resolution must not create competing Person identities. | L2-A02 Person invariants. fileciteturn107file0L2-L2 |
| Business domains do not directly modify Person. | A | Student merge cannot directly mutate the Person aggregate. | L2-A02 Canonical Ownership. fileciteturn106file0L2-L2 |
| Person merge is owned by A02. | A | Any Person-side duplicate resolution remains an A02 business responsibility. | L2-A02 Person Merge. fileciteturn108file0L2-L2 |
| Person merge preserves historical information. | A | Person-side identity history cannot be destroyed by Student resolution. | L2-A02 Person Merge. fileciteturn108file0L2-L2 |

L3 applies these rules; it does not create new merge rules.

---

## 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Canonical Student Aggregate | Existing operational Student record(s) | One Student is selected as the canonical survivor. | Canonical Student remains operational. | Student Lifecycle | L2-C02 WF-C02-16.7 / 16.13. fileciteturn103file0L2-L2 fileciteturn104file0L2-L2 |
| Merged Student Record | Existing duplicate Student record(s) | Duplicate record is removed from operational use through archival. | Archived duplicate Student record. | Student Lifecycle | L2-C02 WF-C02-16.7 / 16.13. fileciteturn103file0L2-L2 fileciteturn104file0L2-L2 |
| Student lifecycle history | Existing historical records | Histories are preserved through the merge. | Historical lineage retained. | Student Lifecycle | L2-C02 WF-C02-16.8 / 16.13. fileciteturn103file0L2-L2 |
| Person | Existing canonical Person | Student merge does not re-own or independently change Person lifecycle. | Existing Person lifecycle remains A02-owned. | Person & Identity | L2-A02 canonical ownership and lifecycle. fileciteturn106file0L2-L2 fileciteturn108file0L2-L2 |

## 13.1 Student Lifecycle State Rule

C017 does not invent a new Student lifecycle state.

The duplicate Student record becomes archived after successful merge according to the locked C02 merge workflow.

The canonical survivor remains the authoritative Student.

## 13.2 Person Lifecycle Rule

A02 defines the Person lifecycle independently as:

`Draft → Created → Verified → Active → Inactive → Archived → Anonymized → Deleted`.

C017 does not change that lifecycle or create a Student-owned Person state. fileciteturn108file0L2-L2

---

## 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `StudentMerged` | **Student Lifecycle** | Downstream domains requiring canonical Student-reference reconciliation | Communicates completion of the canonical Student merge outcome. | L2-C02 WF-C02-16.9 / 16.13. fileciteturn103file0L2-L2 fileciteturn104file0L2-L2 |
| `PersonMerged` | **Person & Identity** | Business domains consuming canonical Person identity | Communicates an A02-owned Person merge when the separate Person merge process applies. | L2-A02 Published Events. fileciteturn109file0L2-L2 |

### Event Ownership Constraint

`StudentMerged` is not renamed to `PersonMerged`.

`PersonMerged` is not republished by Student Lifecycle.

Each event remains owned by its canonical publisher.

A02 explicitly states that downstream business domains do not directly modify the Person aggregate. fileciteturn106file0L2-L2

## 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Student Lifecycle | Participating downstream domains | Canonical Student merge outcome through approved Service Contracts or `StudentMerged` | Reconcile Student references to the canonical survivor. | L2-C02 WF-C02-16.12 / 16.13. fileciteturn103file0L2-L2 fileciteturn104file0L2-L2 |
| Person & Identity | Business domains consuming identity | Canonical Person identity merge outcome through A02-owned contracts/events | Reconcile canonical Person identity where a Person merge is independently required. | L2-A02 Canonical Ownership / Published Events. fileciteturn106file0L2-L2 fileciteturn109file0L2-L2 |

The exact technical mechanism, payload, endpoint, message transport or persistence model is intentionally not defined at L3.

## 14.3 Cross-Domain Boundary

Student Lifecycle remains responsible for the Student merge outcome.

Person & Identity remains responsible for any Person merge outcome.

A Student merge must not be treated as implicit authority to modify Person.

A Person merge must not be treated as implicit authority to change Student lifecycle ownership.

---

## 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Review and approve Student merge | **Registrar** | Merge must be permitted by institutional policy and required approval must be completed. | Student Lifecycle |
| Execute approved Student merge | Student Lifecycle | Must follow the approved merge decision and C02 merge rules. | Student Lifecycle |
| Person-side merge, if separately required | Not defined in the retrieved A02 source for this scenario | Must remain under A02 Person merge ownership. | Person & Identity |

The Registrar's merge-review and approval responsibility is explicitly defined by WF-C02-16.4. fileciteturn102file0L2-L2

No additional approval authority is invented.

## 15.2 Separation of Duties, Overrides and Exceptions

The authoritative C02 source establishes required approval but does not define a separate C017-specific separation-of-duty matrix beyond the Registrar review/approval responsibility.

Therefore:

**No additional scenario-specific separation-of-duty rule is defined by the authoritative L2 architecture.**

No escalation or override authority is invented.

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Duplicate detection | Duplicate-detection/identification history required by the merge workflow | Student Lifecycle |
| Merge request | Traceable Student merge request | Student Lifecycle |
| Approval decision | Recorded approval decision | Student Lifecycle |
| Survivor selection | Record of canonical Student selection | Student Lifecycle |
| Attribute reconciliation | Record of reconciliation outcome | Student Lifecycle |
| Reference migration | Traceable business-reference migration outcome | Student Lifecycle |
| Duplicate archival | Traceable archival outcome | Student Lifecycle |
| Event publication | Traceable `StudentMerged` business event outcome | Student Lifecycle |

The listed audit obligations are directly defined by WF-C02-16.14. fileciteturn104file0L2-L2

A02 separately requires full identity auditability and audit preservation for Person merge operations. fileciteturn109file0L2-L2

No additional evidence fields, technical audit structures or permission mechanisms are invented.

---

## 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

C017 is successfully complete when:

- Duplicate Student records have been validated as eligible for merge.
- Institutional merge policy permits the resolution.
- Required approval has been completed.
- One canonical Student survivor has been selected.
- Permitted Student business information has been reconciled.
- Historical enrollment and lifecycle lineage has been preserved.
- Approved Student business references have been migrated to the canonical survivor.
- Duplicate Student records have been archived.
- Audit history is complete.
- `StudentMerged` has been published by Student Lifecycle.
- Required downstream Student-reference reconciliation has been initiated through approved architecture boundaries.
- The Student Aggregate remains internally consistent.

These criteria derive directly from the locked C02 merge workflow's postconditions and completion definition. fileciteturn104file0L2-L2

## 16.2 Postconditions

- One canonical Student remains active/operational.
- Duplicate Student records are archived.
- Student identity and lifecycle ownership remain with Student Lifecycle.
- Historical lineage is preserved.
- Student merge audit history is complete.
- `StudentMerged` is the canonical Student-side merge event.
- Downstream Student references can reconcile to the canonical survivor.
- Person/Identity ownership remains with A02.
- Any separately required Person merge remains governed by A02.

The C02 source explicitly states that one canonical Student remains active, duplicate Student records are archived, historical lineage is preserved, audit history is complete, `StudentMerged` is published and downstream references are synchronized. fileciteturn104file0L2-L2

---

## 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| **OI-C017-01 — Student Merge vs Person Merge Sequencing** | C02 defines Student merge and A02 defines Person merge, but the retrieved authoritative sources do not establish the exact business sequencing when duplicate Student records also correspond to duplicate Person identities. Choosing an order silently could alter cross-domain semantics. | Confirm the authoritative A↔C sequencing/coordination rule before L3 Locking and before any L4 contract is fixed. |
| **OI-C017-02 — Person-Side Business Actor** | C02 explicitly names Registrar for Student merge approval, but the retrieved A02 material does not define a C017-specific actor for Person merge approval. | Confirm the canonical A02 actor/authority if Person merge becomes part of the same institutional outcome. Do not invent a role. |
| **OI-C017-03 — Exact A↔C Business Hand-off** | The sources establish approved contracts/events and canonical publishers, but do not expose a C017-specific synchronous business hand-off defining how Student Lifecycle and Person & Identity coordinate a coupled duplicate-resolution case. | Confirm the authoritative cross-domain contract/hand-off before L4. |
| **OI-C017-04 — Student-to-Person Duplicate Mapping Rule** | A02 owns one canonical Person per human, while C02 resolves duplicate Student records. The retrieved sources do not define the exact authoritative business rule for determining whether two duplicate Students map to the same Person for every case. | Confirm the upstream identity-to-student relationship rule before locking any L4 duplicate-resolution decision logic. |

These issues do **not** justify inventing architecture. The scenario preserves the unambiguous C02 Student merge outcome and A02 Person ownership while explicitly recording the cross-domain gaps.

## 17.2 Assumptions

| Assumption | Basis | Risk if Incorrect |
|---|---|---|
| C017's primary institutional outcome is resolution of duplicate Student lifecycle records. | Frozen Stage 3 title, owner, type and priority; locked WF-C02-16. fileciteturn95file0L2-L2 fileciteturn102file0L2-L2 | Scenario boundary would require upstream change control. |
| One canonical Student survivor is retained rather than creating a replacement Student. | WF-C02-16.7. fileciteturn103file0L2-L2 | Would contradict locked C02 semantics. |
| The canonical survivor retains Student ID and Student Number. | WF-C02-16.7. fileciteturn103file0L2-L2 | Would contradict Student identity invariants. |
| Person/Identity remains A02-owned even when Student duplicates involve the same human. | A02 canonical ownership. fileciteturn106file0L2-L2 | Would create an ownership conflict requiring architecture change control. |
| `StudentMerged` remains the canonical Student-side merge event. | WF-C02-16.9 / 16.13. fileciteturn103file0L2-L2 fileciteturn104file0L2-L2 | Event ownership would require upstream correction. |

Assumptions do not override locked architecture.

## 17.3 Deferred Questions

| Deferred Question | Target Level | Reason for Deferral |
|---|---|---|
| What exact business contract coordinates a coupled Student merge with an A02 Person merge? | L4 | The retrieved L2 sources establish ownership and events but not the detailed cross-domain sequence/contract. |
| What detailed business reconciliation rules apply to every conflicting Student attribute? | L4 | L2 establishes mergeable categories but does not define field-by-field reconciliation semantics. |
| Which downstream domains require Student reference reconciliation for each merge case? | L4 | L2 identifies multiple dependent domains but does not define a C017-specific consumer matrix. |

---

# Validation and Review Classification

## Frozen Scenario Validation

| Check | Result |
|---|---|
| Final ID preserved | **PASS** |
| Title preserved | **PASS** |
| Primary Owner preserved | **PASS** |
| Participating Domains preserved | **PASS** |
| Scenario Type preserved | **PASS** |
| Priority preserved | **PASS** |
| Derivation preserved | **PASS** |
| Status = Draft | **PASS** |

The frozen Section C register confirms all of these C017 values. fileciteturn95file0L2-L2

## L2-C02 Validation

C017 directly applies `WF-C02-16 — Student Merge & Duplicate Resolution Workflow`.

The workflow establishes:

- duplicate validation;
- same-Tenant requirement;
- canonical survivor selection;
- merge approval;
- mergeable business data;
- reference migration;
- duplicate archival;
- `StudentMerged`;
- downstream synchronization;
- audit requirements;
- terminal outcomes. fileciteturn103file0L2-L2 fileciteturn104file0L2-L2

## L2-A02 Validation

A02 establishes:

- one canonical Person per human within a Tenant;
- immutable Person identity;
- business-domain reference rather than ownership;
- prohibition on downstream direct Person modification;
- Person duplicate detection;
- Person merge ownership;
- `PersonMerged`. fileciteturn107file0L2-L2 fileciteturn108file0L2-L2 fileciteturn109file0L2-L2

C017 does not override those rules.

## Event Validation

Canonical Student event:

**`StudentMerged`**

Canonical publisher:

**Student Lifecycle**.

Canonical Person event, when a separate Person merge applies:

**`PersonMerged`**

Canonical publisher:

**Person & Identity**.

The events remain distinct and retain their canonical publishers. fileciteturn104file0L2-L2 fileciteturn109file0L2-L2

## Lifecycle Validation

No new lifecycle state is invented.

The duplicate Student becomes archived following successful merge.

The canonical Student remains operational.

Person lifecycle remains A02-owned and is not redefined by C017.

## Ownership Validation

- Student Aggregate → Student Lifecycle.
- Duplicate Student resolution → Student Lifecycle.
- Person → Person & Identity.
- Identity → Person & Identity.
- Student downstream consumers → consumers only.
- Person downstream consumers → consumers only.

## Implementation-Leakage Validation

The scenario contains no:

- API definitions;
- payload schemas;
- database design;
- classes/Business Data Structures/services;
- programming language;
- framework;
- UI design;
- deployment architecture;
- infrastructure;
- technical access control/IAM;
- queues;
- technical retries;
- rollback mechanics;
- server exception handling.

## 17-Section Template Validation

All mandatory v2.0 sections are present:

1. Scenario Metadata
2. Scenario Purpose
3. Business Outcome
4. Scenario Boundary and Scope
5. Ownership and Participating Domains
6. Business Actors, Roles and Responsibilities
7. Preconditions and Entry Criteria
8. Trigger
9. Main Business Flow
10. Alternate Business Flows
11. Failure, Interruption and Business Recovery
12. Business Rules and Decision Constraints
13. State and Lifecycle Impact
14. Business Events and Cross-Domain Collaboration
15. Business Authorization, Approval and Audit Obligations
16. Completion Criteria and Postconditions
17. Open Issues, Assumptions and Deferred Questions

The locked v2.0 template defines these 17 sections and explicitly requires unresolved upstream ambiguity to be recorded rather than silently resolved. fileciteturn110file0L2-L2 fileciteturn112file0L2-L2

# Final Classification

**PASS WITH ISSUES — Draft**

C017 is complete as an L3 Draft and is grounded in the actual frozen C017 semantics plus the locked C02 and A02 architecture.

The material cross-domain questions are **explicitly recorded in Section 17** rather than guessed. In particular, the document does not silently decide whether a Student merge must precede, follow, or be coupled with an A02 Person merge.

**Status remains `Draft`.**

AI authoring does not constitute Architecture Approval or Locking.
