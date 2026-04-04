---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#65-summary
chunk_level: summary
chunk_type: prose
heading: Types of volumes
token_count: 54
summary: # mount C:\\Data\\foo from the host, but only if that directory already exists - name: example-volume hostPath: path: \"C:\\\\Data\\\\foo\" # directory location on host type: Directory # this field is...
---

# mount C:\\Data\\foo from the host, but only if that directory already exists
- name: example-volume
hostPath:
path: "C:\\\\Data\\\\foo" # directory location on host
type: Directory # this field is optional
`