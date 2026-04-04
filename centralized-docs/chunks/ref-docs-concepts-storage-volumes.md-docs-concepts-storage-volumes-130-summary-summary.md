---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#130-summary
chunk_level: summary
chunk_type: prose
heading: Out-of-tree volume plugins
token_count: 127
summary: #### Migrating to CSI drivers from in-tree plugins FEATURE STATE: `Kubernetes v1.25 [stable]` The `CSIMigration` feature directs operations against existing in-tree plugins to corresponding CSI...
---

#### Migrating to CSI drivers from in-tree plugins
FEATURE STATE:
`Kubernetes v1.25 [stable]`
The `CSIMigration` feature directs operations against existing in-tree
plugins to corresponding CSI plugins (which are expected to be installed and configured).
As a result, operators do not have to make any
configuration changes to existing Storage Classes, PersistentVolumes, or PersistentVolumeClaims
(referring to in-tree plugins) when transitioning to a CSI driver that supersedes an in-tree plugin.
#### Note:
Existing PVs created by an in-tree volume plugin can still be used in the future without any configuration