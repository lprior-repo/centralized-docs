---
doc_id: usage-example
chunk_id: usage-example#13
chunk_level: summary
chunk_type: prose
heading: Integration with AI Assistants
token_count: 134
summary:         name: 'search_docs',.         arguments: { query, limit }
---





      params: {
        name: 'search_docs',
        arguments: { query, limit }
      }
    });

    let output = '';
    mcp.stdout.on('data', (data) => {
      output += data.toString();
    });

    mcp.on('close', (code) => {
      if (code === 0) {
        resolve(JSON.parse(output));
      } else {
        reject(new Error(`MCP server exited with code ${code}`));
      }
    });

    mcp.stdin.write(request + '\n');
    mcp.stdin.end();
  });
}

// Example usage
queryDocs('rust functional programming').then((results) => {
