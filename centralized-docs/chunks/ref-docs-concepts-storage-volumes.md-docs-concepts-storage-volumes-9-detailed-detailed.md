---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#9-detailed
chunk_level: detailed
chunk_type: prose
heading: Types of volumes
token_count: 1015
summary: The volume is resolved at Pod startup, depending on which `pullPolicy` value is provided: `Always`The kubelet always attempts to pull the reference. If the pull fails, the kubelet sets the Pod to...
---

The volume is resolved at Pod startup, depending on which `pullPolicy` value is
provided:
`Always`The kubelet always attempts to pull the reference. If the pull fails,
the kubelet sets the Pod to `Failed`.`Never`The kubelet never pulls the reference and only uses a local image or artifact.
The Pod becomes `Failed` if any layers of the image aren't already present locally,
or if the manifest for that image isn't already cached.`IfNotPresent`The kubelet pulls if the reference isn't already present on disk. The Pod becomes
`Failed` if the reference isn't present and the pull fails.
The volume gets re-resolved if the Pod gets deleted and recreated, which means
that new remote content will become available on Pod recreation. A failure to
resolve or pull the image during Pod startup will block containers from starting
and may add significant latency. Failures will be retried using normal volume
backoff and will be reported on the Pod reason and message.
The types of objects that may be mounted by this volume are defined by the
container runtime implementation on a host machine. At a minimum, they must include
all valid types supported by the container image field. The OCI object gets
mounted in a single directory (`spec.containers[\*].volumeMounts[\*].mountPath`)
and will be mounted read-only.
Besides that:
* [`subPath`](/docs/concepts/storage/volumes/#using-subpath) or
[`subPathExpr`](/docs/concepts/storage/volumes/#using-subpath-expanded-environment)
mounts for containers (`spec.containers[\*].volumeMounts[\*].subPath`, `spec.containers[\*].volumeMounts[\*].subPathExpr`)
are only supported from Kubernetes v1.33.
* The field `spec.securityContext.fsGroupChangePolicy` has no effect on this
volume type.
* The [`AlwaysPullImages` Admission Controller](/docs/reference/access-authn-authz/admission-controllers/#alwayspullimages)
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
portable manner without manually scheduling Pods to nodes. The system is aware
of the volume's node constraints by looking at the node affinity on the PersistentVolume.
However, `local` volumes are subject to the availability of the underlying
node and are not suitable for all applications. If a node becomes unhealthy,
then the `local` volume becomes inaccessible to the Pod. The Pod using this volume
is unable to run.