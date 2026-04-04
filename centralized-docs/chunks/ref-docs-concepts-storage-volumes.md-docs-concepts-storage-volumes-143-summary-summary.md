---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#143-summary
chunk_level: summary
chunk_type: prose
heading: Read-only mounts
token_count: 127
summary: For example, consider a Pod that mounts the hosts `/mnt` as a `hostPath` volume. If there is another filesystem mounted read-write on `/mnt/&lt;SUBMOUNT&gt;` (such as tmpfs, NFS, or USB storage), the...
---

For example, consider a Pod that mounts the hosts `/mnt` as a `hostPath` volume. If
there is another filesystem mounted read-write on `/mnt/&lt;SUBMOUNT&gt;` (such as tmpfs,
NFS, or USB storage), the volume mounted into the container(s) will also have a writeable
`/mnt/&lt;SUBMOUNT&gt;`, even if the mount itself was specified as read-only.
### Recursive read-only mounts
FEATURE STATE:
`Kubernetes v1.33 [stable]`(enabled by default)
Recursive read-only mounts can be enabled by setting the