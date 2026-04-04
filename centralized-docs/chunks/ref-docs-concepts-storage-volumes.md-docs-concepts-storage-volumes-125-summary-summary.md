---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#125-summary
chunk_level: summary
chunk_type: prose
heading: Out-of-tree volume plugins
token_count: 127
summary: You must also be using a CSI driver that supports or requires secret data during node-initiated storage resize operations. * `nodePublishSecretRef`: A reference to the secret object containing...
---

You must also be using a CSI driver that supports or requires secret data during
node-initiated storage resize operations.
* `nodePublishSecretRef`: A reference to the secret object containing
sensitive information to pass to the CSI driver to complete the CSI
`NodePublishVolume` call. This field is optional and may be empty if no
secret is required. If the secret object contains more than one secret, all
secrets are passed.
* `nodeStageSecretRef`: A reference to the secret object containing
sensitive information to pass to the CSI driver to complete the CSI
`NodeStageVolume` call.