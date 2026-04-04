---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#63-summary
chunk_level: summary
chunk_type: prose
heading: Types of volumes
token_count: 63
summary: # mount /data/foo, but only if that directory already exists hostPath: path: /data/foo # directory location on host type: Directory # this field is optional ` ``` ``` ` --- # This manifest mounts...
---

# mount /data/foo, but only if that directory already exists
hostPath:
path: /data/foo # directory location on host
type: Directory # this field is optional
`
```
```
`
---
# This manifest mounts C:\\Data\\foo on the host as C:\\foo, inside the