---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#20-standard
chunk_level: standard
chunk_type: prose
heading: Types of volumes
token_count: 510
summary: does also work for this volume source like for container images. The following fields are available for the `image` type: `reference`Artifact reference to be used. For example, you could specify...
---

does also work for this volume source like for container images.
The following fields are available for the `image` type:
`reference`Artifact reference to be used. For example, you could specify
`registry.k8s.io/conformance:v1.35.0` to load the
files from the Kubernetes conformance test image. Behaves in the same way as
`pod.spec.containers[\*].image`. Pull secrets will be assembled in the same way
as for the container image by looking up node credentials, service account image
pull secrets, and Pod spec image pull secrets. This field is optional to allow
higher level config management to default or override container images in
workload controllers like Deployments and StatefulSets.
[More info about container images](/docs/concepts/containers/images/).`pullPolicy`Policy for pulling OCI objects. Possible values are: `Always`, `Never`, or
`IfNotPresent`. Defaults to `Always` if `:latest` tag is specified, or
`IfNotPresent` otherwise.
See the [*Use an Image Volume With a Pod*](/docs/tasks/configure-pod-container/image-volumes/)
example for more details on how to use the volume source.
### iscsi
An `iscsi` volume allows an existing iSCSI (SCSI over IP) volume to be mounted
into your Pod. Unlike `emptyDir`, which is erased when a Pod is removed, the
contents of an `iscsi` volume are preserved, and the volume is merely
unmounted. This means that an iscsi volume can be pre-populated with data, and
that data can be shared between Pods.
#### Note:
You must have your own iSCSI server running with the volume created before you can use it.
A feature of iSCSI is that it can be mounted as read-only by multiple consumers
simultaneously. This means that you can pre-populate a volume with your dataset
and then serve it in parallel from as many Pods as you need. Unfortunately,
iSCSI volumes can only be mounted by a single consumer in read-write mode.
Simultaneous writers are not allowed.
### local
A `local` volume represents a mounted local storage device such as a disk,
partition or directory.
Local volumes can only be used as a statically created PersistentVolume. Dynamic
provisioning is not supported.
Compared to `hostPath` volumes, `local` volumes are used in a durable and