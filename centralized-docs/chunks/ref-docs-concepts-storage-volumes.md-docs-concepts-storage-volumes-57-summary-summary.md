---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#57-summary
chunk_level: summary
chunk_type: table
heading: Types of volumes
token_count: 122
summary: |`Directory`|A directory must exist at the given path.| |`FileOrCreate`|If nothing exists at the given path, an empty file will be created there as needed with permission set to 0644, having the same...
---

|`Directory`|A directory must exist at the given path.|
|`FileOrCreate`|If nothing exists at the given path, an empty file will be created there as needed with permission set to 0644, having the same group and ownership with Kubelet.|
|`File`|A file must exist at the given path.|
|`Socket`|A UNIX socket must exist at the given path.|
|`CharDevice`|*(Linux nodes only)* A character device must exist at the given path.|
|`BlockDevice`|*(Linux nodes only)*