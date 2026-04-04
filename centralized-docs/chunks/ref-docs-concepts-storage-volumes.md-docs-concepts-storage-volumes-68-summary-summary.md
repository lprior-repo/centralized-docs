---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#68-summary
chunk_level: summary
chunk_type: prose
heading: Types of volumes
token_count: 89
summary: a file named `/var/local/aaa/1.txt` inside that directory (as seen from the host); if something already exists at that path and isn't a regular file, the Pod fails. Here's the example manifest: ```...
---

a file named `/var/local/aaa/1.txt` inside that directory
(as seen from the host); if something already exists at
that path and isn't a regular file, the Pod fails.
Here's the example manifest:
```
`apiVersion: v1
kind: Pod
metadata:
name: test-webserver
spec:
os: { name: linux }
nodeSelector:
kubernetes.io/os: linux
containers: