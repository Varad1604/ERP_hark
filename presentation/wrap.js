const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

const files = [
    { md: 'SutraERP_Architecture_Workflow_Context_Baseline.md', html: 'SutraERP_Architecture_Workflow_Context_Baseline.html' },
    { md: 'SutraERP_Level_1_Architecture_Locked.md', html: 'SutraERP_Level_1_Architecture_Locked.html' }
];

const template = (title, body) => `<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>${title}</title>
  <style>
    * {
      box-sizing: border-box;
    }

    body {
      margin: 0;
      padding: 32px;
      background: #ffffff;
      color: #191919;
      font-family:
        Inter,
        -apple-system,
        BlinkMacSystemFont,
        "Segoe UI",
        sans-serif;
    }

    .container {
      max-width: 960px;
      margin: 0 auto;
    }

    h1 {
      margin: 0 0 20px;
      font-size: 30px;
      line-height: 1.2;
      font-weight: 700;
    }

    h2 {
      margin: 34px 0 16px;
      color: #37352f;
      font-size: 22px;
      font-weight: 700;
      border-bottom: 1px solid #e8e8e5;
      padding-bottom: 8px;
    }

    h3 {
      margin: 24px 0 12px;
      color: #37352f;
      font-size: 17px;
      font-weight: 700;
    }

    h4, h5, h6 {
      margin: 20px 0 10px;
      color: #37352f;
      font-weight: 700;
    }

    p {
      margin: 0 0 16px;
      color: #333333;
      font-size: 15px;
      line-height: 1.6;
    }

    ul, ol {
      margin: 0 0 16px;
      color: #333333;
      font-size: 15px;
      line-height: 1.6;
      padding-left: 24px;
    }

    li {
      margin-bottom: 8px;
    }

    a {
      color: #25613a;
      text-decoration: none;
      font-weight: 600;
    }

    a:hover {
      text-decoration: underline;
    }

    blockquote {
      padding: 18px 20px;
      border-left: 4px solid #37352f;
      border-radius: 4px 8px 8px 4px;
      background: #f7f7f5;
      margin: 0 0 20px;
    }

    blockquote p {
      margin: 0;
      font-size: 14px;
      font-weight: 600;
      line-height: 1.6;
      color: #191919;
    }

    table {
      border-collapse: collapse;
      width: 100%;
      margin-bottom: 34px;
    }

    th, td {
      border: 1px solid #e8e8e5;
      padding: 14px 16px;
      text-align: left;
      font-size: 14px;
    }

    th {
      background: #fafaf9;
      color: #999999;
      font-size: 11px;
      font-weight: 700;
      letter-spacing: 0.07em;
      text-transform: uppercase;
    }

    td {
      color: #292929;
    }

    pre {
      background: #f1f1ef;
      border-radius: 8px;
      padding: 16px;
      overflow-x: auto;
      margin: 0 0 20px;
    }

    code {
      font-family: monospace;
      background: #f1f1ef;
      padding: 3px 6px;
      border-radius: 4px;
      font-size: 13px;
      color: #db4b4b;
    }

    pre code {
      background: transparent;
      padding: 0;
      color: #37352f;
      font-size: 14px;
    }
    
    pre code.language-mermaid {
      background: transparent;
      padding: 0;
    }

    hr {
      border: 0;
      border-top: 1px solid #e8e8e5;
      margin: 40px 0;
    }

    img {
      max-width: 100%;
      height: auto;
      border-radius: 8px;
      border: 1px solid #e8e8e5;
      margin-bottom: 16px;
    }
    
    /* Center mermaid diagrams */
    .mermaid {
      display: flex;
      justify-content: center;
      margin: 30px 0;
      background: transparent;
    }
  </style>
</head>
<body>
  <main class="container">
    ${body}
  </main>
  
  <script type="module">
    import mermaid from 'https://cdn.jsdelivr.net/npm/mermaid@10/dist/mermaid.esm.min.mjs';
    mermaid.initialize({ startOnLoad: false, theme: 'default' });
    
    // Marked puts mermaid in <pre><code class="language-mermaid">
    // We need to unwrap or convert them so mermaid can render properly.
    document.addEventListener("DOMContentLoaded", async () => {
      const mermaidNodes = document.querySelectorAll('code.language-mermaid');
      for (const node of mermaidNodes) {
        const pre = node.parentElement;
        const div = document.createElement('div');
        div.className = 'mermaid';
        div.textContent = node.textContent;
        pre.parentNode.replaceChild(div, pre);
      }
      await mermaid.run({ querySelector: '.mermaid' });
    });
  </script>
</body>
</html>`;

for (const file of files) {
    const mdPath = path.join(__dirname, file.md);
    const htmlPath = path.join(__dirname, file.html);
    
    if (fs.existsSync(mdPath)) {
        console.log("Converting " + file.md);
        const bodyContent = execSync('npx marked "' + mdPath + '"').toString();
        const titleName = file.md.replace('.md', '').replace(/_/g, ' ');
        const wrappedContent = template(titleName, bodyContent);
        fs.writeFileSync(htmlPath, wrappedContent);
        console.log("Processed " + file.html);
    } else {
        console.log("File " + file.md + " not found.");
    }
}
