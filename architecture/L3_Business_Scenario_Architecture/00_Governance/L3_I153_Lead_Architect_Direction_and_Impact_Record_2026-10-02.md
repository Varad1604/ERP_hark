# L3-I-153 — Lead Architect Direction and Controlled-Successor Gate

**Decision scope:** Issue #136 / draft architecture PR #139  
**Decision date:** 2026-10-02  
**Status:** LEAD ARCHITECT DESIGN DIRECTION APPROVED; independent affected-domain review and controlled L3 successor lock PENDING  
**Proposed successor:** `L3-I-153` v1.1; retains the original locked v1.0 parent until normal change control accepts a corrected successor  
**Authority evidence:** In the SutraERP W5 project conversation, following an explicit yes/no request to approve the *limited interpretation of I13 primary ownership with C05 retaining all qualification authority*, the Lead Architect replied “go ahead and do it.” This records approval of **that bounded design direction**, not an independent Examination or Student Lifecycle domain review or approval of runtime contracts.

## 1. Verified finding and authority

Frozen `CAN-L3-I-158` maps to final `L3-I-153` under the approved Phase-4 five-position offset. Its title, P0 priority, C/H/I participants and **post-authoritative-completion downstream evidence** purpose remain canonical. The historical L3 v1.0 body conflicts with higher-level locked L2-I13 and L2-C05 by assigning Examination-side Academic Completion institutional qualification authority. Draft L4-I-153 explicitly records this upstream blocker.

Locked L2-I13 owns its Examination-derived completion-evidence position and the existing `ExaminationCompletionEvidencePrepared` handoff to C05; its post-conferment supersession/withdrawal/restoration follows the 2026-08-17 approved origin-sensitive order. C05 alone owns institutional qualification decisions and C05-owned publication, correction/revocation/no-change and exit. C04's `QualificationEligible` and C05's later `QualificationAwarded` remain distinct owner facts.

## 2. Explicitly approved bounded design direction

1. Preserve final L3-I-153 identity, scenario title, C/H/I participation, original **after-C05-authoritative-completion** premise, P0 priority, 1,084-scenario corpus size and the historical Section-I primary-owner metadata.
2. Interpret the Section-I primary-owner metadata **strictly as stewardship of the I13 evidence contribution**, NOT ownership of the whole C05 qualification outcome, C05's decision to publish qualification information, downstream H publishing, or any invented joint completion aggregate.
3. Describe L3-I-153 as a multi-owner, cross-domain business-level evidence/traceability scenario. I13 supplies only its independent owner-verifiable Examination evidence to C05 under the existing L2 boundary. C05 remains the sole authority for institutional qualification outcomes and their authorized downstream handoff. A cross-domain narrative does not itself confer runtime orchestration, a new event, an entitlement, a query API or a publisher.
4. Preserve participating H capabilities solely through **their own existing approved receiving boundaries**. H01 may handle officially approved credentials using its own credential-ready/consent/publication governance when a real authorized source is available. H06 must receive C08's exact `AlumniTransitionCompleted` before establishing a separate Alumni Profile; no raw I13 evidence is H06 transition proof.
5. Preserve post-conferment causal direction: an Examination change causes I13 to re-evaluate and publish its superseding existing evidence position; only C05 decides qualification correction/revocation/no-change. A C05-originated qualification-only correction does not automatically change I13 evidence.
6. No new event (`CompletionEvidencePublished` or other), aggregate, policy, authorization role, direct I13→H qualification handoff, C05→I13 award subscription, UI reviewer action or released technical transport is approved by this direction.

## 3. Impact and risk

- **Strength:** Restores higher-level domain authority while preserving the frozen scenario identity, the genuine post-completion purpose and historical L6 observable/noninteractive disposition.
- **Key risk:** Section-I's historical primary-owner label for a post-C05 downstream scenario can be misunderstood as executable cross-domain coordinator authority. Its permitted role is therefore limited to source-evidence stewardship and documentary collaboration; **C05 alone controls its qualification outputs**. A future runtime system may realize only those parts supported by released owner contracts.
- **Rejected alternative:** Rewriting this scenario as the *initial* I13→C05 evidence-preparation workflow. That is a separately defined L2 interaction and would change this frozen scenario's trigger and downstream outcome.
- **No ID conflict:** `CAN-L3-I-153 → L3-I-148` and `CAN-L3-I-158 → L3-I-153`; previous speculation about a numbering defect is superseded.

## 4. Controlled acceptance gate, not automatically waived

This record documents the Lead Architect's explicit design-direction approval. It **does not fabricate** the Examination or C05 domain architects' independent review. Before promoting the proposed successor to `Locked`, the affected-domain review must confirm the limited historical primary-owner reading, independent C05 qualification publication and permitted H participation; material objections must be resolved by the Lead Architect through this change-control process.

Only after accepted locked L3 successor text may the currently blocked Draft L4-I-153 be re-derived and independently validated. Inspect indirect L5 and the L6 W-I13 trace (observable, **PROHIBITED**, no manual completion button). No currently verified backend publisher/read schema or real W5 J06 handoff is inferred.

## 5. Scope of repository update

PR #139 retains the immutable original locked v1.0 scenario on Architecture main, adds the scoped successor proposal and this explicit direction record. No unrelated L3 scenario, existing locked L1/L2, L4 spec, L6 disposition or frontend owner module is modified. Merge of **proposal/decision documentation** is not approval/lock of the physical canonical L3 successor.

## 6. Evidence and traceability

- `L3_Final_Canonical_Scenario_ID_Register.md` §8; frozen `Section_I_141_197_Canonical_Register.md`.
- Locked `L2-I13_Academic_Completion_CGPA_and_Completion_Evidence_Workflows.md`, including approved I-XI-02 origin-sensitive ordering.
- Locked `L2-C05_Qualification_Completion_and_Student_Exit_Workflows.md` and `L3-C-039_Qualification_Completion_and_Degree_Conferment.md`.
- H01 credential publication and H06 `AlumniTransitionCompleted` source gates; original Locked L3-I-153 and blocked Draft L4-I-153.
- Constitution Volume 5 (proposal, impact, review, decision, update, validation) and full-corpus L3 lock §5 (controlled successor).
- https://github.com/DhruvNaik26/SutraERP_Architecture/issues/136 and https://github.com/DhruvNaik26/SutraERP_Architecture/pull/139.

**Design direction:** LEAD ARCHITECT APPROVED  
**Independent affected-domain review:** NOT VERIFIED  
**New locked L3 successor:** PENDING CONTROLLED ACCEPTANCE  
**Runtime source contract / integrated W5 processing:** NOT VERIFIED
