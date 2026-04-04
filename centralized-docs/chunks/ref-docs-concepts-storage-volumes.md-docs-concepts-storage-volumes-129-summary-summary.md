---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#129-summary
chunk_level: summary
chunk_type: prose
heading: Out-of-tree volume plugins
token_count: 125
summary: These operations differ for each host operating system. For Linux worker nodes, containerized CSI node plugins are typically deployed as privileged containers. For Windows worker nodes, privileged...
---

These operations
differ for each host operating system. For Linux worker nodes, containerized CSI node
plugins are typically deployed as privileged containers. For Windows worker nodes,
privileged operations for containerized CSI node plugins are supported using
[csi-proxy](https://github.com/kubernetes-csi/csi-proxy), a community-managed,
stand-alone binary that needs to be pre-installed on each Windows node.
For more details, refer to the deployment guide of the CSI plugin you wish to deploy.
#### Migrating to CSI drivers from in-tree plugins
FEATURE STATE:
`Kubernetes v1.25 [stable]`