---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#122-summary
chunk_level: summary
chunk_type: prose
heading: Out-of-tree volume plugins
token_count: 123
summary: * `controllerPublishSecretRef`: A reference to the secret object containing sensitive information to pass to the CSI driver to complete the CSI `ControllerPublishVolume` and...
---

* `controllerPublishSecretRef`: A reference to the secret object containing
sensitive information to pass to the CSI driver to complete the CSI
`ControllerPublishVolume` and `ControllerUnpublishVolume` calls. This field is
optional, and may be empty if no secret is required. If the Secret
contains more than one secret, all secrets are passed.
* `nodeExpandSecretRef`: A reference to the secret containing sensitive
information to pass to the CSI driver to complete the CSI
`NodeExpandVolume` call. This field is optional and may be empty if no
secret is required.