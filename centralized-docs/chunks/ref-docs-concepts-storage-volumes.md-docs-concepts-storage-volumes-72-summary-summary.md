---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#72-summary
chunk_level: summary
chunk_type: prose
heading: Types of volumes
token_count: 85
summary: `apiVersion: v1 kind: Pod metadata: name: image-volume spec: containers: - name: shell command: [\"sleep\", \"infinity\"] image: debian volumeMounts: - name: volume mountPath: /volume volumes: - name:...
---

`apiVersion: v1
kind: Pod
metadata:
name: image-volume
spec:
containers:
- name: shell
command: ["sleep", "infinity"]
image: debian
volumeMounts:
- name: volume
mountPath: /volume
volumes:
- name: volume
image:
reference: quay.io/crio/artifact:v2
pullPolicy: IfNotPresent
`