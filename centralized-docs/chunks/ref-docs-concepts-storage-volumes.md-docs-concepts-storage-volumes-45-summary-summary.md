---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#45-summary
chunk_level: summary
chunk_type: prose
heading: Types of volumes
token_count: 111
summary: The `gitRepo` volume plugin is deprecated and is disabled by default. To provision a Pod that has a Git repository mounted, you can mount an [`emptyDir`](#emptydir) volume into an [init...
---

The `gitRepo` volume plugin is deprecated and is disabled by default.
To provision a Pod that has a Git repository mounted, you can mount an
[`emptyDir`](#emptydir) volume into an [init container](/docs/concepts/workloads/pods/init-containers/)
that clones the repo using Git, then mount the [EmptyDir](#emptydir) into the Pod's container.
You can restrict the use of `gitRepo` volumes in your cluster using
[policies](/docs/concepts/policy/), such as