---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#107-summary
chunk_level: summary
chunk_type: prose
heading: Using subPath
token_count: 65
summary: The `hostPath` volume takes the `Pod` name from the `downwardAPI`. The host directory `/var/log/pods/pod1` is mounted at `/logs` in the container. ``` `apiVersion: v1 kind: Pod metadata: name: pod1...
---

The `hostPath` volume takes the `Pod` name from the `downwardAPI`.
The host directory `/var/log/pods/pod1` is mounted at `/logs` in the container.
```
`apiVersion: v1
kind: Pod
metadata:
name: pod1
spec:
containers: