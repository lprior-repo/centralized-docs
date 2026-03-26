---
doc_id: tutorial/docs-tasks-run-application-scale-stateful-set.md/docs-tasks-run-application-scale-stateful-set
chunk_id: tutorial/docs-tasks-run-application-scale-stateful-set.md/docs-tasks-run-application-scale-stateful-set#5-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 76
summary: Otherwise, edit that field with `kubectl edit`: ``` `kubectl edit statefulsets &lt;stateful-set-name&gt; ` ``` Or use `kubectl patch`: ``` `kubectl patch statefulsets &lt;stateful-set-name&gt; -p...
---

Otherwise, edit that field with `kubectl edit`:
```
`kubectl edit statefulsets &lt;stateful-set-name&gt;
`
```
Or use `kubectl patch`:
```
`kubectl patch statefulsets &lt;stateful-set-name&gt; -p '{"spec":{"replicas":&lt;new-replicas&gt;}}'
`
```