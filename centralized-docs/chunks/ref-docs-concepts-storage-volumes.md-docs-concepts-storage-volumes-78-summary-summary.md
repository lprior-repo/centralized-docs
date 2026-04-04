---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#78-summary
chunk_level: summary
chunk_type: prose
heading: Types of volumes
token_count: 123
summary: * The field `spec.securityContext.fsGroupChangePolicy` has no effect on this volume type. * The [`AlwaysPullImages` Admission...
---

* The field `spec.securityContext.fsGroupChangePolicy` has no effect on this
volume type.
* The [`AlwaysPullImages` Admission Controller](/docs/reference/access-authn-authz/admission-controllers/#alwayspullimages)
does also work for this volume source like for container images.
The following fields are available for the `image` type:
`reference`Artifact reference to be used. For example, you could specify
`registry.k8s.io/conformance:v1.35.0` to load the
files from the Kubernetes conformance test image. Behaves in the same way as