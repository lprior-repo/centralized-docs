---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#33-standard
chunk_level: standard
chunk_type: prose
heading: Out-of-tree volume plugins
token_count: 501
summary: to work, even for brand-new volumes. The actual storage management now happens through the CSI driver. The operations and features that are supported include: provisioning/delete, attach/detach,...
---

to work, even for brand-new volumes. The actual storage management now happens through
the CSI driver.
The operations and features that are supported include:
provisioning/delete, attach/detach, mount/unmount, and resizing of volumes.
In-tree plugins that support `CSIMigration` and have a corresponding CSI driver implemented
are listed in [Types of Volumes](#volume-types).
### flexVolume (deprecated)
FEATURE STATE:
`Kubernetes v1.23 [deprecated]`
FlexVolume is an out-of-tree plugin interface that uses an exec-based model to interface
with storage drivers. The FlexVolume driver binaries must be installed in a pre-defined
volume plugin path on each node, and in some cases, the control plane nodes as well.
Pods interact with FlexVolume drivers through the `flexVolume` in-tree volume plugin.
The following FlexVolume [plugins](https://github.com/Microsoft/K8s-Storage-Plugins/tree/master/flexvolume/windows),
deployed as PowerShell scripts on the host, support Windows nodes:
* [SMB](https://github.com/microsoft/K8s-Storage-Plugins/tree/master/flexvolume/windows/plugins/microsoft.com~smb.cmd)
* [iSCSI](https://github.com/microsoft/K8s-Storage-Plugins/tree/master/flexvolume/windows/plugins/microsoft.com~iscsi.cmd)
#### Note:
FlexVolume is deprecated. Using an out-of-tree CSI driver is the recommended way to integrate external storage with Kubernetes.
Maintainers of the FlexVolume driver should implement a CSI Driver and help migrate users of FlexVolume drivers to CSI.
Users of FlexVolume should move their workloads to use the equivalent CSI Driver.
#### Caution:
Mount propagation is a low-level feature that does not work consistently on all
volume types. The Kubernetes project recommends only using mount propagation with `hostPath`
or memory-backed `emptyDir` volumes. See
[Kubernetes issue #95049](https://github.com/kubernetes/kubernetes/issues/95049)
for more context.
Mount propagation allows for sharing volumes mounted by a container to
other containers in the same Pod, or even to other Pods on the same node.
Mount propagation of a volume is controlled by the `mountPropagation` field
in `containers[\*].volumeMounts`. Its values are:
* `None` - This volume mount will not receive any subsequent mounts
that are mounted to this volume or any of its subdirectories by the host.