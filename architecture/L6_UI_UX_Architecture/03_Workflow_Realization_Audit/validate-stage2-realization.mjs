import fs from "node:fs";
import path from "node:path";

const root = process.cwd();
const resultDir = path.join(root, "architecture/L6_UI_UX_Architecture/03_Workflow_Realization_Audit/worker-results");
const l3Root = path.join(root, "architecture/L3_Business_Scenario_Architecture/05_Phase5_Parallel_Authoring_and_Verification/01_Scenario_Documents");
const tracePath = path.join(root, "architecture/L6_UI_UX_Architecture/L6_Screen_Journey_Traceability_Register_DRAFT.md");

const sections = {A:62,B:33,C:75,D:57,E:75,F:85,G:320,H:81,I:192,J:104};
const expectedWorkers = {
  A:["L3-A-001","L3-A-062",62], B:["L3-B-001","L3-B-033",33],
  C:["L3-C-001","L3-C-075",75], D:["L3-D-001","L3-D-057",57],
  E:["L3-E-001","L3-E-075",75], F:["L3-F-001","L3-F-085",85],
  G1:["L3-G-001","L3-G-077",77], G2:["L3-G-078","L3-G-154",77],
  G3:["L3-G-155","L3-G-251",97], G4:["L3-G-252","L3-G-320",69],
  H:["L3-H-001","L3-H-081",81], I1:["L3-I-001","L3-I-082",82],
  I2:["L3-I-083","L3-I-144",62], I3:["L3-I-145","L3-I-192",48],
  J:["L3-J-001","L3-J-104",104]
};
const issuePrefixes = [
  "Implementation defect","L6 UX architecture defect","L5 feasibility/integration issue",
  "L4/L3/L2/L1 architecture issue","Review-process issue","Implementation-local improvement"
];
const dispositions = new Set([
  "INTERACTIVE-WORKFLOW","REVIEW-APPROVAL-WORKFLOW","TRACKING-WORKFLOW",
  "EXCEPTION-RECOVERY-WORKFLOW","CROSS-WORKSPACE-HANDOFF",
  "SYSTEM-BACKGROUND-OBSERVABLE","NO-DIRECT-END-USER-INTERACTION"
]);
const mockValues = new Set(["REQUIRED","NOT-REQUIRED","PROHIBITED"]);
const archetypes = new Set([
  "entity-360","search-worklist","master-detail-queue","guided-flow",
  "governed-lifecycle-action","roster-bulk-action","schedule-calendar",
  "financial-ledger","request-document-tracking","configuration-master-data",
  "analytics-monitoring","persona-home-dashboard"
]);

function walk(dir) {
  return fs.readdirSync(dir, {withFileTypes:true}).flatMap(e => {
    const p = path.join(dir,e.name);
    return e.isDirectory() ? walk(p) : [p];
  });
}
const canonical = new Map();
for (const file of walk(l3Root).filter(f => /L3-[A-J]-\d{3}_.+\.md$/.test(f))) {
  const text = fs.readFileSync(file,"utf8");
  const id = (text.match(/\*\*Scenario ID:\*\*\s*(L3-[A-J]-\d{3})/)||[])[1];
  const title = (text.match(/^#\s+(.+)$/m)||[])[1];
  if (id && title) canonical.set(id,title.trim());
}
const traceText = fs.readFileSync(tracePath,"utf8");
const workspaceKeys = new Set([...traceText.matchAll(/\b(W-[A-J]-?\d{2}|W-[A-J]\d{2}|S-\d{2})\b/g)].map(m=>m[1]));

const files = fs.existsSync(resultDir)
  ? fs.readdirSync(resultDir).filter(f => /^STAGE2_WORKER_.+_RESULT\.md$/.test(f))
  : [];

const rows = [];
const errors = [];
for (const file of files) {
  const text = fs.readFileSync(path.join(resultDir,file),"utf8");
  const workerId = (text.match(/^workerId:\s*(.+)$/m)||[])[1]?.trim();
  const assignedRange = (text.match(/^assignedRange:\s*(.+)$/m)||[])[1]?.trim();
  const assignedCount = Number((text.match(/^assignedCount:\s*(\d+)$/m)||[])[1]);
  const processedCount = Number((text.match(/^processedCount:\s*(\d+)$/m)||[])[1]);
  const missingCount = Number((text.match(/^missingCount:\s*(\d+)$/m)||[])[1]);
  const duplicateCount = Number((text.match(/^duplicateCount:\s*(\d+)$/m)||[])[1]);
  const resultStatus = (text.match(/^resultStatus:\s*(.+)$/m)||[])[1]?.trim();
  if (!workerId || !expectedWorkers[workerId]) errors.push(`${file}: invalid/missing workerId`);
  else {
    const [first,last,count] = expectedWorkers[workerId];
    if (assignedRange !== `${first} … ${last}`) errors.push(`${file}: assignedRange mismatch`);
    if (assignedCount !== count || processedCount !== count || missingCount !== 0 || duplicateCount !== 0)
      errors.push(`${file}: worker header counts/status inconsistent`);
    if (resultStatus !== "COMPLETE-FOR-MASTER-REVIEW") errors.push(`${file}: invalid resultStatus`);
  }
  for (const line of text.split(/\r?\n/)) {
    if (!/^\|\s*L3-[A-J]-\d{3}\s*\|/.test(line)) continue;
    const cells = line.split("|").slice(1,-1).map(x=>x.trim().replace(/\\\|/g,"|"));
    if (cells.length !== 14) { errors.push(`${file}: expected 14 columns, got ${cells.length}`); continue; }
    const [id,title,workspace,profiles,disp,label,discovery,outcome,mock,arch,owner,trace,maturity,issue] = cells;
    rows.push({file,id,title,workspace,profiles,disp,label,discovery,outcome,mock,arch,owner,trace,maturity,issue});
  }
}

const byId = new Map();
for (const r of rows) {
  if (!canonical.has(r.id)) errors.push(`${r.file}: unknown canonical ID ${r.id}`);
  else if (canonical.get(r.id) !== r.title) errors.push(`${r.file}: title mismatch for ${r.id}`);
  if (byId.has(r.id)) errors.push(`duplicate scenario row ${r.id}: ${byId.get(r.id).file}, ${r.file}`);
  else byId.set(r.id,r);
  if (!workspaceKeys.has(r.workspace)) errors.push(`${r.file}: unknown workspace/shared-surface key ${r.workspace} for ${r.id}`);
  if (!dispositions.has(r.disp)) errors.push(`${r.file}: invalid disposition ${r.disp} for ${r.id}`);
  if (!mockValues.has(r.mock)) errors.push(`${r.file}: invalid mockInteraction ${r.mock} for ${r.id}`);
  for (const a of r.arch.split(";").map(x=>x.trim()).filter(Boolean)) if (!archetypes.has(a)) errors.push(`${r.file}: invalid archetype ${a} for ${r.id}`);
  if (!r.profiles || !r.label || !r.discovery || !r.outcome || !r.owner || !r.trace || !r.maturity) errors.push(`${r.file}: required field empty for ${r.id}`);
  if (r.maturity !== "SCENARIO-REALIZATION-MAPPED") errors.push(`${r.file}: unsupported Stage-2 maturity ${r.maturity} for ${r.id}`);
  if (r.issue && !issuePrefixes.some(p => r.issue.startsWith(p + ":"))) errors.push(`${r.file}: invalid openIssue classification for ${r.id}`);
  if (/\bL[345]-[A-J]-\d{3}\b/.test(r.label)) errors.push(`${r.file}: architecture ID leaked into userFacingLabel for ${r.id}`);
}

const expected=[];
for (const [s,c] of Object.entries(sections)) for(let n=1;n<=c;n++) expected.push(`L3-${s}-${String(n).padStart(3,"0")}`);
const missing=expected.filter(id=>!byId.has(id));
const finalMode=process.argv.includes("--final");
if (canonical.size !== 1084) errors.push(`canonical L3 count is ${canonical.size}, expected 1084`);
if (files.length !== 15) errors.push(`worker result file count is ${files.length}, expected 15`);
if (finalMode && rows.length !== 1084) errors.push(`final row count is ${rows.length}, expected 1084`);
if (finalMode && missing.length) errors.push(`missing ${missing.length} scenarios: ${missing.slice(0,20).join(", ")}`);

const distribution={};
for(const r of rows) distribution[r.disp]=(distribution[r.disp]||0)+1;
console.log(JSON.stringify({
  canonicalCount:canonical.size,
  workerResultFiles:files.length,
  rowCount:rows.length,
  uniqueIds:byId.size,
  missingCount:missing.length,
  errorCount:errors.length,
  distribution,
  errors:errors.slice(0,100)
},null,2));
if(errors.length) process.exitCode=1;
