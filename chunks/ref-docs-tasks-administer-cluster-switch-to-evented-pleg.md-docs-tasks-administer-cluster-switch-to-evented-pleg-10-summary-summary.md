---
doc_id: ref/docs-tasks-administer-cluster-switch-to-evented-pleg.md/docs-tasks-administer-cluster-switch-to-evented-pleg
chunk_id: ref/docs-tasks-administer-cluster-switch-to-evented-pleg.md/docs-tasks-administer-cluster-switch-to-evented-pleg#10-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 87
summary: ``` `[crio.runtime] enable\_pod\_events: true ` ``` Your Kubernetes server must be at or later than version 1.26. To check the version, enter `kubectl version`. 4. Verify that the kubelet is using...
---

```
`[crio.runtime]
enable\_pod\_events: true
`
```
Your Kubernetes server must be at or later than version 1.26.
To check the version, enter `kubectl version`.
4. Verify that the kubelet is using event-based container stage change monitoring.
To check, look for the term `EventedPLEG` in the kubelet logs.
The output should be similar to this: