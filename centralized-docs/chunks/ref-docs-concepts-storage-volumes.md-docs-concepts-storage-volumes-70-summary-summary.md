---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#70-summary
chunk_level: summary
chunk_type: prose
heading: Types of volumes
token_count: 46
summary: # Ensure the file directory is created. path: /var/local/aaa type: DirectoryOrCreate - name: myfile hostPath: path: /var/local/aaa/1.txt type: FileOrCreate `
---

# Ensure the file directory is created.
path: /var/local/aaa
type: DirectoryOrCreate
- name: myfile
hostPath:
path: /var/local/aaa/1.txt
type: FileOrCreate
`