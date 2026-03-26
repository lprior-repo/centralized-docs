---
doc_id: ref/docs-tasks-administer-cluster-switch-to-evented-pleg.md/docs-tasks-administer-cluster-switch-to-evented-pleg
chunk_id: ref/docs-tasks-administer-cluster-switch-to-evented-pleg.md/docs-tasks-administer-cluster-switch-to-evented-pleg#11-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 78
summary: ``` `I0314 11:10:13.909915 1105457 feature\_gate.go:249] feature gates: &amp;{map[EventedPLEG:true]} ` ``` If you have set `--v` to 4 and above, you might see more entries that indicate that the...
---

```
`I0314 11:10:13.909915 1105457 feature\_gate.go:249] feature gates: &amp;{map[EventedPLEG:true]}
`
```
If you have set `--v` to 4 and above, you might see more entries that indicate
that the kubelet is using event-based container state monitoring.