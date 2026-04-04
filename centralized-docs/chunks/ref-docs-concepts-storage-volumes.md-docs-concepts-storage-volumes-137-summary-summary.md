---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#137-summary
chunk_level: summary
chunk_type: prose
heading: Out-of-tree volume plugins
token_count: 125
summary: in `containers[\*].volumeMounts`. Its values are: * `None` - This volume mount will not receive any subsequent mounts that are mounted to this volume or any of its subdirectories by the host. In a...
---

in `containers[\*].volumeMounts`. Its values are:
* `None` - This volume mount will not receive any subsequent mounts
that are mounted to this volume or any of its subdirectories by the host.
In a similar fashion, no mounts created by the container will be visible on
the host. This is the default mode.
This mode is equal to `rprivate` mount propagation as described in
[`mount(8)`](https://man7.org/linux/man-pages/man8/mount.8.html)
However, the CRI runtime may choose `rslave` mount propagation (i.e.,