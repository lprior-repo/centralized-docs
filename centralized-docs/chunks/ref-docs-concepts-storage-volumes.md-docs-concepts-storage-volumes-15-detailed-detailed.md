---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#15-detailed
chunk_level: detailed
chunk_type: prose
heading: Out-of-tree volume plugins
token_count: 1013
summary: ## Resources The storage medium (such as Disk or SSD) of an `emptyDir` volume is determined by the medium of the filesystem holding the kubelet root dir (typically `/var/lib/kubelet`). There is no...
---

## Resources
The storage medium (such as Disk or SSD) of an `emptyDir` volume is determined by the
medium of the filesystem holding the kubelet root dir (typically
`/var/lib/kubelet`). There is no limit on how much space an `emptyDir` or
`hostPath` volume can consume, and no isolation between containers or
Pods.
To learn about requesting space using a resource specification, see
[how to manage resources](/docs/concepts/configuration/manage-resources-containers/).
## Out-of-tree volume plugins
The out-of-tree volume plugins include
[Container Storage Interface](/docs/concepts/storage/volumes/#csi) (CSI), and also
FlexVolume (which is deprecated). These plugins enable storage vendors to create custom storage plugins
without adding their plugin source code to the Kubernetes repository.
Previously, all volume plugins were "in-tree". The "in-tree" plugins were built, linked, compiled,
and shipped with the core Kubernetes binaries. This meant that adding a new storage system to
Kubernetes (a volume plugin) required checking code into the core Kubernetes code repository.
Both CSI and FlexVolume allow volume plugins to be developed independently of
the Kubernetes code base, and deployed (installed) on Kubernetes clusters as
extensions.
For storage vendors looking to create an out-of-tree volume plugin, please refer
to the [volume plugin FAQ](https://github.com/kubernetes/community/blob/master/sig-storage/volume-plugin-faq.md).
### csi
[Container Storage Interface](https://github.com/container-storage-interface/spec/blob/master/spec.md)
(CSI) defines a standard interface for container orchestration systems (like
Kubernetes) to expose arbitrary storage systems to their container workloads.
Please read the [CSI design proposal](https://git.k8s.io/design-proposals-archive/storage/container-storage-interface.md)
for more information.
#### Note:
Support for CSI spec versions 0.2 and 0.3 is deprecated in Kubernetes
v1.13 and will be removed in a future release.
#### Note:
CSI drivers may not be compatible across all Kubernetes releases.
Please check the specific CSI driver's documentation for supported
deployment steps for each Kubernetes release and a compatibility matrix.
Once a CSI-compatible volume driver is deployed on a Kubernetes cluster, users
may use the `csi` volume type to attach or mount the volumes exposed by the
CSI driver.
A `csi` volume can be used in a Pod in three different ways:
* through a reference to a [PersistentVolumeClaim](#persistentvolumeclaim)
* with a [generic ephemeral volume](/docs/concepts/storage/ephemeral-volumes/#generic-ephemeral-volumes)
* with a [CSI ephemeral volume](/docs/concepts/storage/ephemeral-volumes/#csi-ephemeral-volumes)
if the driver supports that
The following fields are available to storage administrators to configure a CSI
persistent volume:
* `driver`: A string value that specifies the name of the volume driver to use.
This value must correspond to the value returned in the `GetPluginInfoResponse`
by the CSI driver as defined in the
[CSI spec](https://github.com/container-storage-interface/spec/blob/master/spec.md#getplugininfo).
It is used by Kubernetes to identify which CSI driver to call out to, and by
CSI driver components to identify which PV objects belong to the CSI driver.
* `volumeHandle`: A string value that uniquely identifies the volume. This value
must correspond to the value returned in the `volume.id` field of the
`CreateVolumeResponse` by the CSI driver as defined in the
[CSI spec](https://github.com/container-storage-interface/spec/blob/master/spec.md#createvolume).
The value is passed as `volume\_id` in all calls to the CSI volume driver when
referencing the volume.
* `readOnly`: An optional boolean value indicating whether the volume is to be
"ControllerPublished" (attached) as read-only. Default is false. This value is passed
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