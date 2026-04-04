---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#95-summary
chunk_level: summary
chunk_type: prose
heading: Types of volumes
token_count: 118
summary: details. ### portworxVolume (deprecated) FEATURE STATE: `Kubernetes v1.25 [deprecated]` A `portworxVolume` is an elastic block storage layer that runs hyperconverged with Kubernetes....
---

details.
### portworxVolume (deprecated)
FEATURE STATE:
`Kubernetes v1.25 [deprecated]`
A `portworxVolume` is an elastic block storage layer that runs hyperconverged with
Kubernetes. [Portworx](https://portworx.com/use-case/kubernetes-storage/) fingerprints storage
in a server, tiers based on capabilities, and aggregates capacity across multiple servers.
Portworx runs in-guest in virtual machines or on bare metal Linux nodes.
A `portworxVolume` can be dynamically created through Kubernetes, or it can also