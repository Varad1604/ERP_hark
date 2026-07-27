const fs = require('fs');

function processDiagram(text) {
    let originalText = text;
    
    text = text.replace(/[^\w\s]*\+"\s*/g, '--> ');
    text = text.replace(/[^\w\s]*"o[^\w\s]*"\[^\w\s]*"\\s*/g, '--> ');
    text = text.replace(/[^\w\s]*""[^\w\s]*"\[^\w\s]*"\\s*/g, '--> ');
    text = text.replace(/[^\w\s]*",\s*/g, '');
    
    // Catch down arrows and other common ascii arrows
    text = text.replace(/↓/g, '--> ');
    text = text.replace(/->/g, '--> ');
    text = text.replace(/=>/g, '--> ');
    
    const lines = text.trim().split('\\n').map(l => l.trim()).filter(l => l);
    
    let isFlow = false;
    let newLines = [];
    let lastNode = null;
    
    for (let i = 0; i < lines.length; i++) {
        let line = lines[i];
        if (line.startsWith('-->')) {
            isFlow = true;
            let target = line.substring(3).trim();
            if (target === '') {
                if (i + 1 < lines.length) {
                    target = lines[i+1];
                    i++;
                }
            }
            if (lastNode && target) {
                newLines.push("    " + lastNode.replace(/[\\[\\]()]/g, '') + " --> " + target.replace(/[\\[\\]()]/g, ''));
                lastNode = target;
            }
        } else {
            if (line.includes('-->')) {
                isFlow = true;
                const parts = line.split('-->').map(p => p.trim());
                for (let j = 0; j < parts.length - 1; j++) {
                     if (parts[j] && parts[j+1]) {
                         newLines.push("    " + parts[j].replace(/[\\[\\]()]/g, '') + " --> " + parts[j+1].replace(/[\\[\\]()]/g, ''));
                     }
                }
                lastNode = parts[parts.length - 1];
            } else {
                lastNode = line;
            }
        }
    }
    
    if (isFlow && newLines.length > 0) {
        return "```mermaid\\nflowchart TD\\n" + newLines.join('\\n') + "\\n```";
    }
    
    return "``` text\\n" + originalText + "\\n```";
}

function processFile(filePath) {
    if (!fs.existsSync(filePath)) return;
    let content = fs.readFileSync(filePath, 'utf8');
    const regex = /\`\`\` text\\s*\\n([\\s\\S]*)\\n\\s*\`\`\`/g;
    content = content.replace(regex, (match, p1) => {
        return processDiagram(p1);
    });
    fs.writeFileSync(filePath, content, 'utf8');
    console.log("Processed " + filePath);
}

processFile('SutraERP_Level_1_Architecture_Locked.md');
processFile('SutraERP_Architecture_Workflow_Context_Baseline.md');
