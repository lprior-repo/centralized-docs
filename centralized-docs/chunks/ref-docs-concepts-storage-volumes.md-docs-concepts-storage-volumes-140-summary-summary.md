---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#140-summary
chunk_level: summary
chunk_type: prose
heading: Out-of-tree volume plugins
token_count: 128
summary: * `Bidirectional` - This volume mount behaves the same as the `HostToContainer` mount. In addition, all volume mounts created by the container will be propagated back to the host and to all...
---

* `Bidirectional` - This volume mount behaves the same as the `HostToContainer` mount.
In addition, all volume mounts created by the container will be propagated
back to the host and to all containers of all Pods that use the same volume.
A typical use case for this mode is a Pod with a FlexVolume or CSI driver, or
a Pod that needs to mount something on the host using a `hostPath` volume.
This mode is equal to `rshared` mount propagation as described in the
[`mount(8)`](https://man7.org/linux/man-pages/man8/mount.8.html)