// Canonical source/output paths for the presentation conversion tools.
// Resolved from the repository root, not from the tools/ directory.
// Closes integrity finding INT-022.

const path = require('path');

const REPO_ROOT = path.resolve(__dirname, '..');

const DOCS = [
    {
        id: 'L0',
        md: path.join(REPO_ROOT, 'architecture', 'L0_Business_Vision',
            'SutraERP_Architecture_Workflow_Context_Baseline.md'),
        html: path.join(REPO_ROOT, 'generated', 'html',
            'SutraERP_Architecture_Workflow_Context_Baseline.html'),
        title: 'SutraERP Architecture Workflow Context Baseline'
    },
    {
        id: 'L1',
        md: path.join(REPO_ROOT, 'architecture', 'L1_Business_Domain_Architecture',
            'SutraERP_L1_Business_Domain_Architecture_Locked.md'),
        html: path.join(REPO_ROOT, 'generated', 'html',
            'SutraERP_Level_1_Architecture_Locked.html'),
        title: 'SutraERP Level 1 Architecture (Locked)'
    }
];

const HTML_DIR = path.join(REPO_ROOT, 'generated', 'html');

module.exports = { REPO_ROOT, DOCS, HTML_DIR };
