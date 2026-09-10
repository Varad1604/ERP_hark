# SutraERP L5 Governance

This directory is the canonical navigation point for L5 governance.

## Structure

```text
governance/
├── README.md
├── instruction/   # daily authoring/review kit required by all team members
└── reference/     # supporting governance, provenance and non-daily decisions
```

### `instruction/`
Use this folder for active L5 authoring and review. It contains the standards, template, validation rules, canonical 105-specification register, workload allocation, execution sequence, naming rules, traceability rules and ADR controls that all members may need while working.

Start with `instruction/README.md`.

### `reference/`
Use this folder when checking planning provenance, approval history, coverage evidence, superseded registers, pilot decisions or repository-structure decisions.

A file being in `reference/` does **not** reduce its architectural authority or status. The distinction is frequency of operational use, not authority.

## Placement Rule

No substantive L5 governance file shall be added directly to this root unless it is a navigation file. New governance material must be classified into `instruction/` or `reference/` according to its operational role.

Canonical filenames, statuses, identifiers and architectural meaning are unchanged by this folder organization.
