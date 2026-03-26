---
doc_id: ref/docs-tasks-debug-debug-cluster-topology.md/docs-tasks-debug-debug-cluster-topology
chunk_id: ref/docs-tasks-debug-debug-cluster-topology.md/docs-tasks-debug-debug-cluster-topology#11-summary
chunk_level: summary
chunk_type: prose
heading: Examine system logs
token_count: 104
summary: ### Examine the memory manager state on a node Let us first deploy a sample `Guaranteed` pod whose specification is as follows: ``` `apiVersion: v1 kind: Pod metadata: name: guaranteed spec:...
---

### Examine the memory manager state on a node
Let us first deploy a sample `Guaranteed` pod whose specification is as follows:
```
`apiVersion: v1
kind: Pod
metadata:
name: guaranteed
spec:
containers:
- name: guaranteed
image: consumer
imagePullPolicy: Never
resources:
limits:
cpu: "2"
memory: 150Gi
requests:
cpu: "2"
memory: 150Gi
command: ["sleep","infinity"]
`
```