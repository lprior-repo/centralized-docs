---
doc_id: ref/docs-concepts-workloads-pods-pod-hostname.md/docs-concepts-workloads-pods-pod-hostname
chunk_id: ref/docs-concepts-workloads-pods-pod-hostname.md/docs-concepts-workloads-pods-pod-hostname#15-summary
chunk_level: summary
chunk_type: prose
heading: Hostname with pod's hostnameOverride
token_count: 72
summary: ``` `apiVersion: v1 kind: Pod metadata: name: busybox-2-busybox-example-domain spec: hostnameOverride: busybox-2.busybox.example.domain containers: - image: busybox:1.28 command: - sleep - \"3600\"...
---

```
`apiVersion: v1
kind: Pod
metadata:
name: busybox-2-busybox-example-domain
spec:
hostnameOverride: busybox-2.busybox.example.domain
containers:
- image: busybox:1.28
command:
- sleep
- "3600"
name: busybox
`
```