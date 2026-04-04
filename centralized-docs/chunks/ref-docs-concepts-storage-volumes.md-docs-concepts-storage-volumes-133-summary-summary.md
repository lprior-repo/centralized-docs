---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#133-summary
chunk_level: summary
chunk_type: prose
heading: Out-of-tree volume plugins
token_count: 105
summary: are listed in [Types of Volumes](#volume-types). ### flexVolume (deprecated) FEATURE STATE: `Kubernetes v1.23 [deprecated]` FlexVolume is an out-of-tree plugin interface that uses an exec-based model...
---

are listed in [Types of Volumes](#volume-types).
### flexVolume (deprecated)
FEATURE STATE:
`Kubernetes v1.23 [deprecated]`
FlexVolume is an out-of-tree plugin interface that uses an exec-based model to interface
with storage drivers. The FlexVolume driver binaries must be installed in a pre-defined
volume plugin path on each node, and in some cases, the control plane nodes as well.
Pods interact with FlexVolume drivers through the `flexVolume` in-tree volume plugin.