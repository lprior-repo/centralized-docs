---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#29-standard
chunk_level: standard
chunk_type: prose
heading: Out-of-tree volume plugins
token_count: 502
summary: #### Note: CSI drivers may not be compatible across all Kubernetes releases. Please check the specific CSI driver's documentation for supported deployment steps for each Kubernetes release and a...
---

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