---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#151-summary
chunk_level: summary
chunk_type: prose
heading: Read-only mounts
token_count: 128
summary: When this property is recognized by kubelet and kube-apiserver, the `.status.containerStatuses[\*].volumeMounts[\*].recursiveReadOnly` field is set to either `Enabled` or `Disabled`. ####...
---

When this property is recognized by kubelet and kube-apiserver,
the `.status.containerStatuses[\*].volumeMounts[\*].recursiveReadOnly` field is set to either
`Enabled` or `Disabled`.
#### Implementations
**Note:** This section links to third party projects that provide functionality required by Kubernetes. The Kubernetes project authors aren't responsible for these projects, which are listed alphabetically. To add a project to this list, read the [content guide](/docs/contribute/style/content-guide/#third-party-content) before submitting a change. [More information.](#third-party-content-disclaimer)