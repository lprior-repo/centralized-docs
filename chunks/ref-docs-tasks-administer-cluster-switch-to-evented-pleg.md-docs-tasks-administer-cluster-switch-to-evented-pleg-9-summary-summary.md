---
doc_id: ref/docs-tasks-administer-cluster-switch-to-evented-pleg.md/docs-tasks-administer-cluster-switch-to-evented-pleg
chunk_id: ref/docs-tasks-administer-cluster-switch-to-evented-pleg.md/docs-tasks-administer-cluster-switch-to-evented-pleg#9-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 79
summary: If it is enabled, the output should be similar to the following: ``` `enable\_pod\_events = true ` ``` To enable it, start the CRI-O daemon with the flag `--enable-pod-events=true` or use a dropin...
---

If it is enabled, the output should be similar to the following:
```
`enable\_pod\_events = true
`
```
To enable it, start the CRI-O daemon with the flag `--enable-pod-events=true` or
use a dropin config with the following lines:
```
`[crio.runtime]
enable\_pod\_events: true
`
```