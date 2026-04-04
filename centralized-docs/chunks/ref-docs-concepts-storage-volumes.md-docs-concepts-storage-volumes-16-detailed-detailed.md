---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#16-detailed
chunk_level: detailed
chunk_type: prose
heading: Out-of-tree volume plugins
token_count: 1024
summary: Default is false. This value is passed to the CSI driver via the `readonly` field in the `ControllerPublishVolumeRequest`. * `fsType`: If the PV's `VolumeMode` is `Filesystem`, then this field may be...
---

Default is false. This value is passed
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
You must also be using a CSI driver that supports or requires secret data during
node-initiated storage resize operations.
* `nodePublishSecretRef`: A reference to the secret object containing
sensitive information to pass to the CSI driver to complete the CSI
`NodePublishVolume` call. This field is optional and may be empty if no
secret is required. If the secret object contains more than one secret, all
secrets are passed.
* `nodeStageSecretRef`: A reference to the secret object containing
sensitive information to pass to the CSI driver to complete the CSI
`NodeStageVolume` call. This field is optional and may be empty if no secret
is required. If the Secret contains more than one secret, all secrets
are passed.#### CSI raw block volume support
FEATURE STATE:
`Kubernetes v1.18 [stable]`
Vendors with external CSI drivers can implement raw block volume support
in Kubernetes workloads.
You can set up your
[PersistentVolume/PersistentVolumeClaim with raw block volume support](/docs/concepts/storage/persistent-volumes/#raw-block-volume-support)
as usual, without any CSI-specific changes.
#### CSI ephemeral volumes
FEATURE STATE:
`Kubernetes v1.25 [stable]`
You can directly configure CSI volumes within the Pod
specification. Volumes specified in this way are ephemeral and do not
persist across Pod restarts. See
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
plugins are typically deployed as privileged containers. For Windows worker nodes,
privileged operations for containerized CSI node plugins are supported using
[csi-proxy](https://github.com/kubernetes-csi/csi-proxy), a community-managed,
stand-alone binary that needs to be pre-installed on each Windows node.
For more details, refer to the deployment guide of the CSI plugin you wish to deploy.
#### Migrating to CSI drivers from in-tree plugins