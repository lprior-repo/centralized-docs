---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#124-summary
chunk_level: summary
chunk_type: prose
heading: Out-of-tree volume plugins
token_count: 126
summary: `nodeExpandSecretRef` field, and have it available by default. Kubernetes releases prior to v1.25 did not include this support. * Enable the [feature...
---

`nodeExpandSecretRef` field, and have it available by default. Kubernetes releases
prior to v1.25 did not include this support.
* Enable the [feature gate](/docs/reference/command-line-tools-reference/feature-gates-removed/)
named `CSINodeExpandSecret` for each kube-apiserver and for the kubelet on every
node. Since Kubernetes version 1.27, this feature has been enabled by default
and no explicit enablement of the feature gate is required.
You must also be using a CSI driver that supports or requires secret data during
node-initiated storage resize operations.