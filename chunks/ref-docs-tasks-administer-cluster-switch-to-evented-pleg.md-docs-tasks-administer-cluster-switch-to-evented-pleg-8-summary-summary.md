---
doc_id: ref/docs-tasks-administer-cluster-switch-to-evented-pleg.md/docs-tasks-administer-cluster-switch-to-evented-pleg
chunk_id: ref/docs-tasks-administer-cluster-switch-to-evented-pleg.md/docs-tasks-administer-cluster-switch-to-evented-pleg#8-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 115
summary: 2. Make sure the node is [drained](/docs/tasks/administer-cluster/safely-drain-node/) before proceeding. 3. Start the container runtime with the container event generation enabled. Version 1.7+...
---

2. Make sure the node is [drained](/docs/tasks/administer-cluster/safely-drain-node/) before proceeding.
3. Start the container runtime with the container event generation enabled.
Version 1.7+
Version 1.26+
Check if the CRI-O is already configured to emit CRI events by verifying the configuration,
```
`crio config | grep enable\_pod\_events
`
```
If it is enabled, the output should be similar to the following:
```
`enable\_pod\_events = true
`
```