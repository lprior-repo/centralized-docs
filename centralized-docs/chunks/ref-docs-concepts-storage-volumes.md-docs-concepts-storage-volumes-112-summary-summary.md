---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#112-summary
chunk_level: summary
chunk_type: prose
heading: Out-of-tree volume plugins
token_count: 123
summary: ## Out-of-tree volume plugins The out-of-tree volume plugins include [Container Storage Interface](/docs/concepts/storage/volumes/#csi) (CSI), and also FlexVolume (which is deprecated). These plugins...
---

## Out-of-tree volume plugins
The out-of-tree volume plugins include
[Container Storage Interface](/docs/concepts/storage/volumes/#csi) (CSI), and also
FlexVolume (which is deprecated). These plugins enable storage vendors to create custom storage plugins
without adding their plugin source code to the Kubernetes repository.
Previously, all volume plugins were "in-tree". The "in-tree" plugins were built, linked, compiled,
and shipped with the core Kubernetes binaries. This meant that adding a new storage system to
Kubernetes (a volume plugin) required checking code into the core Kubernetes code repository.