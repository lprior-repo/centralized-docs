---
doc_id: usage-example
chunk_id: usage-example#14
chunk_level: summary
chunk_type: prose
heading: Integration with AI Assistants
token_count: 58
summary: // Example usage. queryDocs('rust functional programming')
---

      }
    });

    mcp.stdin.end();
  });
}

// Example usage
queryDocs('rust functional programming').then((results) => {
  results.results.forEach((doc) => {
    console.log(`- ${doc.title} (score: ${doc.score})`);
  });
});
```

