---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#56-summary
chunk_level: summary
chunk_type: table
heading: Types of volumes
token_count: 92
summary: |Empty string (default) is for backward compatibility, which means that no checks will be performed before mounting the `hostPath` volume.| |`DirectoryOrCreate`|If nothing exists at the given path,...
---

|Empty string (default) is for backward compatibility, which means that no checks will be performed before mounting the `hostPath` volume.|
|`DirectoryOrCreate`|If nothing exists at the given path, an empty directory will be created there as needed with permission set to 0755, having the same group and ownership with Kubelet.|
|`Directory`|A directory must exist at the given path.|
|`FileOrCreate`