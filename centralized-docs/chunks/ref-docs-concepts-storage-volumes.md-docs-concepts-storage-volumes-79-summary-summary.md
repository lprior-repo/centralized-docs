---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#79-summary
chunk_level: summary
chunk_type: prose
heading: Types of volumes
token_count: 118
summary: files from the Kubernetes conformance test image. Behaves in the same way as `pod.spec.containers[\*].image`. Pull secrets will be assembled in the same way as for the container image by looking up...
---

files from the Kubernetes conformance test image. Behaves in the same way as
`pod.spec.containers[\*].image`. Pull secrets will be assembled in the same way
as for the container image by looking up node credentials, service account image
pull secrets, and Pod spec image pull secrets. This field is optional to allow
higher level config management to default or override container images in
workload controllers like Deployments and StatefulSets.
[More info about container images](/docs/concepts/containers/images/).`pullPolicy`Policy for pulling OCI objects.