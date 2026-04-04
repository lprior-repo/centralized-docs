---
doc_id: ref/docs-tasks-debug-debug-cluster-audit.md/docs-tasks-debug-debug-cluster-audit
chunk_id: ref/docs-tasks-debug-debug-cluster-audit.md/docs-tasks-debug-debug-cluster-audit#22-summary
chunk_level: summary
chunk_type: prose
heading: Audit policy
token_count: 55
summary: You can use a minimal audit policy file to log all requests at the `Metadata` level: ``` `# Log all requests at the Metadata level. apiVersion: audit.k8s.io/v1 kind: Policy rules: - level: Metadata `...
---

You can use a minimal audit policy file to log all requests at the `Metadata` level:
```
`# Log all requests at the Metadata level.
apiVersion: audit.k8s.io/v1
kind: Policy
rules:
- level: Metadata
`
```