---
doc_id: ref/docs-tasks-debug-debug-cluster-audit.md/docs-tasks-debug-debug-cluster-audit
chunk_id: ref/docs-tasks-debug-debug-cluster-audit.md/docs-tasks-debug-debug-cluster-audit#21-summary
chunk_level: summary
chunk_type: prose
heading: Audit policy
token_count: 119
summary: # Log all other resources in core and extensions at the Request level. - level: Request resources: - group: \"\" # core API group - group: \"extensions\" # Version of group should NOT be included. # A...
---

# Log all other resources in core and extensions at the Request level.
- level: Request
resources:
- group: "" # core API group
- group: "extensions" # Version of group should NOT be included.
# A catch-all rule to log all other requests at the Metadata level.
- level: Metadata
# Long-running requests like watches that fall under this rule will not
# generate an audit event in RequestReceived.
omitStages:
- "RequestReceived"
`
```
You can use a minimal audit policy file to log all requests at the `Metadata` level: