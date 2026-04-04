---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#144-summary
chunk_level: summary
chunk_type: prose
heading: Read-only mounts
token_count: 119
summary: FEATURE STATE: `Kubernetes v1.33 [stable]`(enabled by default) Recursive read-only mounts can be enabled by setting the `RecursiveReadOnlyMounts` [feature...
---

FEATURE STATE:
`Kubernetes v1.33 [stable]`(enabled by default)
Recursive read-only mounts can be enabled by setting the
`RecursiveReadOnlyMounts` [feature gate](/docs/reference/command-line-tools-reference/feature-gates/)
for kubelet and kube-apiserver, and setting the `.spec.containers[\*].volumeMounts[\*].recursiveReadOnly`
field for a Pod.
The allowed values are:
* `Disabled` (default): no effect.
* `Enabled`: makes the mount recursively read-only.
Needs all the following requirements to be satisfied: