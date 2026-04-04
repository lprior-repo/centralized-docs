---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#19-standard
chunk_level: standard
chunk_type: prose
heading: Types of volumes
token_count: 505
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
`pod.spec.containers[\*].image`.