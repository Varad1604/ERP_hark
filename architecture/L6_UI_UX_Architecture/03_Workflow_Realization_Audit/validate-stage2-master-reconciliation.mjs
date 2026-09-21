import fs from "node:fs";
import path from "node:path";

const root=process.cwd();
const audit=path.join(root,"architecture/L6_UI_UX_Architecture/03_Workflow_Realization_Audit");
const masterPath=path.join(audit,"group-reconciliation/STAGE2_GROUP_15_MASTER_CROSS_GROUP_RECONCILIATION.md");
const registerPath=path.join(root,"architecture/L6_UI_UX_Architecture/L6_Workflow_Realization_Register_DRAFT.md");

const sections={A:62,B:33,C:75,D:57,E:75,F:85,G:320,H:81,I:192,J:104};
const dispositions=new Set([
 "INTERACTIVE-WORKFLOW","REVIEW-APPROVAL-WORKFLOW","TRACKING-WORKFLOW",
 "EXCEPTION-RECOVERY-WORKFLOW","CROSS-WORKSPACE-HANDOFF",
 "SYSTEM-BACKGROUND-OBSERVABLE","NO-DIRECT-END-USER-INTERACTION"
]);
const mocks=new Set(["REQUIRED","NOT-REQUIRED","PROHIBITED"]);
const errors=[];

if(!fs.existsSync(masterPath)) errors.push("missing Group-15 master reconciliation artifact");
if(!fs.existsSync(registerPath)) errors.push("missing Workflow Realization Register");

const expected=[];
for(const [s,c] of Object.entries(sections)) for(let n=1;n<=c;n++) expected.push(`L3-${s}-${String(n).padStart(3,"0")}`);
const expectedSet=new Set(expected);

const master=fs.existsSync(masterPath)?fs.readFileSync(masterPath,"utf8"):"";
const workflows=new Map();
const scenarioToWorkflow=new Map();
const workspaceGroups=new Map();
for(const line of master.split(/\r?\n/)){
 if(!/^\| L6-RWF-\d{4} \|/.test(line)) continue;
 const c=line.split("|").slice(1,-1).map(x=>x.trim());
 if(c.length!==8){errors.push(`master workflow row has ${c.length} columns`);continue;}
 const [id,sourceKey,group,workspace,family,scenarioCol,disp,mock]=c;
 if(workflows.has(id)) errors.push(`duplicate workflow ${id}`);
 if(!dispositions.has(disp)) errors.push(`invalid workflow disposition ${disp} on ${id}`);
 if(!mocks.has(mock)) errors.push(`invalid workflow mock ${mock} on ${id}`);
 if(!/^G\d+-WF-\d{3}$/.test(sourceKey)) errors.push(`invalid source key ${sourceKey} on ${id}`);
 const g=Number(group); if(!(g>=1&&g<=14)) errors.push(`invalid group ${group} on ${id}`);
 (workspaceGroups.get(workspace)??workspaceGroups.set(workspace,new Set()).get(workspace)).add(g);
 const ids=[...scenarioCol.matchAll(/L3-[A-J]-\d{3}/g)].map(m=>m[0]);
 if(!ids.length) errors.push(`no scenarios on ${id}`);
 workflows.set(id,{sourceKey,g,workspace,family,disp,mock,ids});
 for(const sid of ids){
   if(!expectedSet.has(sid)) errors.push(`unexpected scenario ${sid} on ${id}`);
   if(scenarioToWorkflow.has(sid)) errors.push(`duplicate master scenario assignment ${sid}`);
   scenarioToWorkflow.set(sid,id);
 }
}
if(workflows.size!==595) errors.push(`workflow count ${workflows.size}, expected 595`);
for(let n=1;n<=595;n++){
 const id=`L6-RWF-${String(n).padStart(4,"0")}`;
 if(!workflows.has(id)) errors.push(`missing normalized workflow ${id}`);
}
for(const [ws,groups] of workspaceGroups) if(groups.size>1) errors.push(`workspace ${ws} appears across groups ${[...groups].join(",")}`);
if(scenarioToWorkflow.size!==1084) errors.push(`master unique scenario count ${scenarioToWorkflow.size}, expected 1084`);
for(const id of expected) if(!scenarioToWorkflow.has(id)) errors.push(`master missing scenario ${id}`);

const register=fs.existsSync(registerPath)?fs.readFileSync(registerPath,"utf8"):"";
const registerRows=new Map();
for(const line of register.split(/\r?\n/)){
 if(!/^\| L3-[A-J]-\d{3} \|/.test(line)) continue;
 const c=line.split("|").slice(1,-1).map(x=>x.trim());
 if(c.length!==15){errors.push(`register row has ${c.length} columns: ${c[0]??"unknown"}`);continue;}
 const [id,title,workspace,normalized,profiles,disp,label,discovery,outcome,mock,archetypes,owner,trace,maturity,issue]=c;
 if(registerRows.has(id)) errors.push(`duplicate register scenario ${id}`);
 if(!expectedSet.has(id)) errors.push(`unexpected register scenario ${id}`);
 if(!dispositions.has(disp)) errors.push(`invalid register disposition ${disp} for ${id}`);
 if(!mocks.has(mock)) errors.push(`invalid register mock ${mock} for ${id}`);
 if(!title||!workspace||!normalized||!profiles||!label||!discovery||!outcome||!archetypes||!owner||!trace) errors.push(`required register field empty for ${id}`);
 if(maturity!=="MASTER-RECONCILED") errors.push(`invalid maturity ${maturity} for ${id}`);
 if(/\bL[345]-[A-J]-\d{3}\b/.test(label)) errors.push(`architecture ID leaked into user label for ${id}`);
 const m=normalized.match(/^(L6-RWF-\d{4})\s+—\s+(.+)$/);
 if(!m) errors.push(`invalid normalized workflow reference for ${id}: ${normalized}`);
 else {
   const wf=workflows.get(m[1]);
   if(!wf) errors.push(`unknown normalized workflow ${m[1]} for ${id}`);
   else {
     if(!wf.ids.includes(id)) errors.push(`${id} not assigned to ${m[1]} in master catalog`);
     if(wf.workspace!==workspace) errors.push(`workspace mismatch for ${id}: register ${workspace}, workflow ${wf.workspace}`);
     if(wf.family!==m[2]) errors.push(`workflow family mismatch for ${id}`);
   }
 }
 registerRows.set(id,{disp,mock});
}
if(registerRows.size!==1084) errors.push(`register unique row count ${registerRows.size}, expected 1084`);
for(const id of expected) if(!registerRows.has(id)) errors.push(`register missing scenario ${id}`);

const workflowDistribution={},workflowMock={},scenarioDistribution={},scenarioMock={};
for(const w of workflows.values()){workflowDistribution[w.disp]=(workflowDistribution[w.disp]||0)+1;workflowMock[w.mock]=(workflowMock[w.mock]||0)+1;}
for(const r of registerRows.values()){scenarioDistribution[r.disp]=(scenarioDistribution[r.disp]||0)+1;scenarioMock[r.mock]=(scenarioMock[r.mock]||0)+1;}

console.log(JSON.stringify({
 canonicalExpected:1084,
 masterWorkflows:workflows.size,
 masterScenarioAssignments:scenarioToWorkflow.size,
 registerRows:registerRows.size,
 repeatedWorkspaceGroups:[...workspaceGroups].filter(([_,s])=>s.size>1).length,
 workflowDistribution,
 workflowMock,
 scenarioDistribution,
 scenarioMock,
 errorCount:errors.length,
 errors:errors.slice(0,100)
},null,2));
if(errors.length) process.exitCode=1;
