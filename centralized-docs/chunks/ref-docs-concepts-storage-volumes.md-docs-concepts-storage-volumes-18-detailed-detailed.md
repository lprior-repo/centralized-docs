---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#18-detailed
chunk_level: detailed
chunk_type: prose
heading: Read-only mounts
token_count: 1015
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
containers. Familiarity with Linux kernel behavior is strongly recommended.
In addition, any volume mounts created by containers in Pods must be destroyed
(unmounted) by the containers on termination.
## Read-only mounts
A mount can be made read-only by setting the `.spec.containers[\*].volumeMounts[\*].readOnly`
field to `true`.
This does not make the volume itself read-only, but that specific container will
not be able to write to it.
Other containers in the Pod may mount the same volume as read-write.
On Linux, read-only mounts are not recursively read-only by default.
For example, consider a Pod that mounts the hosts `/mnt` as a `hostPath` volume. If
there is another filesystem mounted read-write on `/mnt/&lt;SUBMOUNT&gt;` (such as tmpfs,
NFS, or USB storage), the volume mounted into the container(s) will also have a writeable
`/mnt/&lt;SUBMOUNT&gt;`, even if the mount itself was specified as read-only.
### Recursive read-only mounts
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
* `readOnly` is set to `true`
* `mountPropagation` is unset, or set to `None`
* The host is running with Linux kernel v5.12 or later
* The [CRI-level](/docs/concepts/architecture/cri) container runtime supports recursive read-only mounts
* The OCI-level container runtime supports recursive read-only mounts.
It will fail if any of these is not true.
* `IfPossible`: attempts to apply `Enabled`, and falls back to `Disabled`
if the feature is not supported by the kernel or the runtime class.
Example:
[`storage/rro.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/storage/rro.yaml)![](/images/copycode.svg "Copy storage/rro.yaml to clipboard")
```