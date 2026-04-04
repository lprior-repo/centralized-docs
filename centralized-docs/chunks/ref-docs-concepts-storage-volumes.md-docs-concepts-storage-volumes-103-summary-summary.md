---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#103-summary
chunk_level: summary
chunk_type: prose
heading: Using subPath
token_count: 43
summary: the MySQL database is stored in the volume's `mysql` folder. For example: ``` `apiVersion: v1 kind: Pod metadata: name: my-lamp-site spec: containers:
---

the MySQL database is stored in the volume's `mysql` folder. For example:
```
`apiVersion: v1
kind: Pod
metadata:
name: my-lamp-site
spec:
containers: