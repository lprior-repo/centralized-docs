---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#128-summary
chunk_level: summary
chunk_type: prose
heading: Out-of-tree volume plugins
token_count: 121
summary: [Ephemeral Volumes](/docs/concepts/storage/ephemeral-volumes/#csi-ephemeral-volumes) for more information. For more information on how to develop a CSI driver, refer to the [kubernetes-csi...
---

[Ephemeral Volumes](/docs/concepts/storage/ephemeral-volumes/#csi-ephemeral-volumes)
for more information.
For more information on how to develop a CSI driver, refer to the
[kubernetes-csi documentation](https://kubernetes-csi.github.io/docs/)
#### Windows CSI proxy
FEATURE STATE:
`Kubernetes v1.22 [stable]`
CSI node plugins need to perform various privileged
operations like scanning of disk devices and mounting of file systems. These operations
differ for each host operating system. For Linux worker nodes, containerized CSI node