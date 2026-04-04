---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#34-standard
chunk_level: standard
chunk_type: prose
heading: Out-of-tree volume plugins
token_count: 511
summary: for more context. Mount propagation allows for sharing volumes mounted by a container to other containers in the same Pod, or even to other Pods on the same node. Mount propagation of a volume is...
---

for more context.
Mount propagation allows for sharing volumes mounted by a container to
other containers in the same Pod, or even to other Pods on the same node.
Mount propagation of a volume is controlled by the `mountPropagation` field
in `containers[\*].volumeMounts`. Its values are:
* `None` - This volume mount will not receive any subsequent mounts
that are mounted to this volume or any of its subdirectories by the host.
In a similar fashion, no mounts created by the container will be visible on
the host. This is the default mode.
This mode is equal to `rprivate` mount propagation as described in
[`mount(8)`](https://man7.org/linux/man-pages/man8/mount.8.html)
However, the CRI runtime may choose `rslave` mount propagation (i.e.,
`HostToContainer`) when `rprivate` propagation is not applicable.
cri-dockerd (Docker) is known to choose `rslave` mount propagation when the
mount source contains the Docker daemon's root directory (`/var/lib/docker`).
* `HostToContainer` - This volume mount will receive all subsequent mounts
that are mounted to this volume or any of its subdirectories.
In other words, if the host mounts anything inside the volume mount, the
container will see it mounted there.
Similarly, if any Pod with `Bidirectional` mount propagation to the same
volume mounts anything there, the container with `HostToContainer` mount
propagation will see it.
This mode is equal to `rslave` mount propagation as described in the
[`mount(8)`](https://man7.org/linux/man-pages/man8/mount.8.html)
* `Bidirectional` - This volume mount behaves the same as the `HostToContainer` mount.
In addition, all volume mounts created by the container will be propagated
back to the host and to all containers of all Pods that use the same volume.
A typical use case for this mode is a Pod with a FlexVolume or CSI driver, or
a Pod that needs to mount something on the host using a `hostPath` volume.
This mode is equal to `rshared` mount propagation as described in the
[`mount(8)`](https://man7.org/linux/man-pages/man8/mount.8.html)
#### Warning:
`Bidirectional` mount propagation can be dangerous. It can damage
the host operating system, and therefore, it is allowed only in privileged