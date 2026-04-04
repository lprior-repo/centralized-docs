---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#30-standard
chunk_level: standard
chunk_type: prose
heading: Out-of-tree volume plugins
token_count: 511
summary: to the CSI driver via the `readonly` field in the `ControllerPublishVolumeRequest`. * `fsType`: If the PV's `VolumeMode` is `Filesystem`, then this field may be used to specify the filesystem that...
---

to the CSI driver via the `readonly` field in the `ControllerPublishVolumeRequest`.
* `fsType`: If the PV's `VolumeMode` is `Filesystem`, then this field may be used
to specify the filesystem that should be used to mount the volume. If the
volume has not been formatted and formatting is supported, this value will be
used to format the volume.
This value is passed to the CSI driver via the `VolumeCapability` field of
`ControllerPublishVolumeRequest`, `NodeStageVolumeRequest`, and
`NodePublishVolumeRequest`.
* `volumeAttributes`: A map of string to string that specifies static properties
of a volume. This map must correspond to the map returned in the
`volume.attributes` field of the `CreateVolumeResponse` by the CSI driver as
defined in the [CSI spec](https://github.com/container-storage-interface/spec/blob/master/spec.md#createvolume).
The map is passed to the CSI driver via the `volume\_context` field in the
`ControllerPublishVolumeRequest`, `NodeStageVolumeRequest`, and
`NodePublishVolumeRequest`.
* `controllerPublishSecretRef`: A reference to the secret object containing
sensitive information to pass to the CSI driver to complete the CSI
`ControllerPublishVolume` and `ControllerUnpublishVolume` calls. This field is
optional, and may be empty if no secret is required. If the Secret
contains more than one secret, all secrets are passed.
* `nodeExpandSecretRef`: A reference to the secret containing sensitive
information to pass to the CSI driver to complete the CSI
`NodeExpandVolume` call. This field is optional and may be empty if no
secret is required. If the object contains more than one secret, all
secrets are passed. When you have configured secret data for node-initiated
volume expansion, the kubelet passes that data via the `NodeExpandVolume()`
call to the CSI driver. All supported versions of Kubernetes offer the
`nodeExpandSecretRef` field, and have it available by default. Kubernetes releases
prior to v1.25 did not include this support.
* Enable the [feature gate](/docs/reference/command-line-tools-reference/feature-gates-removed/)
named `CSINodeExpandSecret` for each kube-apiserver and for the kubelet on every
node. Since Kubernetes version 1.27, this feature has been enabled by default
and no explicit enablement of the feature gate is required.