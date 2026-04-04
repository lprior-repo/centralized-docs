---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#101-summary
chunk_level: summary
chunk_type: prose
heading: Using subPath
token_count: 119
summary: backed by tmpfs (a RAM-backed filesystem), so they are never written to non-volatile storage. #### Note: * You must create a Secret in the Kubernetes API before you can use it. * A Secret is always...
---

backed by tmpfs (a RAM-backed filesystem), so they are never written to
non-volatile storage.
#### Note:
* You must create a Secret in the Kubernetes API before you can use it.
* A Secret is always mounted as `readOnly`.
* A container using a Secret as a [`subPath`](#using-subpath) volume mount will not
receive Secret updates.
For more details, see [Configuring Secrets](/docs/concepts/configuration/secret/).
## Using subPath
Sometimes, it is useful to share one volume for multiple uses in a single Pod.