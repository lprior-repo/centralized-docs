---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#40-summary
chunk_level: summary
chunk_type: prose
heading: Types of volumes
token_count: 90
summary: #### emptyDir memory configuration example ``` `apiVersion: v1 kind: Pod metadata: name: test-pd spec: containers: - image: registry.k8s.io/test-webserver name: test-container volumeMounts: -...
---

#### emptyDir memory configuration example
```
`apiVersion: v1
kind: Pod
metadata:
name: test-pd
spec:
containers:
- image: registry.k8s.io/test-webserver
name: test-container
volumeMounts:
- mountPath: /cache
name: cache-volume
volumes:
- name: cache-volume
emptyDir:
sizeLimit: 500Mi
medium: Memory
`
```