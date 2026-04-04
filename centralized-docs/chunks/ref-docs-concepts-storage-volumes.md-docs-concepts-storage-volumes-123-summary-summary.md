---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#123-summary
chunk_level: summary
chunk_type: prose
heading: Out-of-tree volume plugins
token_count: 110
summary: `NodeExpandVolume` call. This field is optional and may be empty if no secret is required. If the object contains more than one secret, all secrets are passed. When you have configured secret data...
---

`NodeExpandVolume` call. This field is optional and may be empty if no
secret is required. If the object contains more than one secret, all
secrets are passed. When you have configured secret data for node-initiated
volume expansion, the kubelet passes that data via the `NodeExpandVolume()`
call to the CSI driver. All supported versions of Kubernetes offer the
`nodeExpandSecretRef` field, and have it available by default. Kubernetes releases
prior to v1.25 did not include this support.