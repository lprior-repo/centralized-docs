---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#139-summary
chunk_level: summary
chunk_type: prose
heading: Out-of-tree volume plugins
token_count: 119
summary: In other words, if the host mounts anything inside the volume mount, the container will see it mounted there. Similarly, if any Pod with `Bidirectional` mount propagation to the same volume mounts...
---

In other words, if the host mounts anything inside the volume mount, the
container will see it mounted there.
Similarly, if any Pod with `Bidirectional` mount propagation to the same
volume mounts anything there, the container with `HostToContainer` mount
propagation will see it.
This mode is equal to `rslave` mount propagation as described in the
[`mount(8)`](https://man7.org/linux/man-pages/man8/mount.8.html)
* `Bidirectional` - This volume mount behaves the same as the `HostToContainer` mount.