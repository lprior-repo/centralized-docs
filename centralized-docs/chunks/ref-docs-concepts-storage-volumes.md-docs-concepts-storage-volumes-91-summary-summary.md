---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#91-summary
chunk_level: summary
chunk_type: prose
heading: Types of volumes
token_count: 93
summary: `apiVersion: v1 kind: Pod metadata: name: test-pd spec: containers: - image: registry.k8s.io/test-webserver name: test-container volumeMounts: - mountPath: /my-nfs-data name: test-volume volumes: -...
---

`apiVersion: v1
kind: Pod
metadata:
name: test-pd
spec:
containers:
- image: registry.k8s.io/test-webserver
name: test-container
volumeMounts:
- mountPath: /my-nfs-data
name: test-volume
volumes:
- name: test-volume
nfs:
server: my-nfs-server.example.com
path: /my-nfs-volume
readOnly: true
`