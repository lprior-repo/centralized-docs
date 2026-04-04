---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#14-standard
chunk_level: standard
chunk_type: table
heading: Types of volumes
token_count: 474
summary: * Access to the host filesystem can expose privileged system credentials (such as for the kubelet) or privileged APIs (such as the container runtime socket) that can be used for container escape or...
---

* Access to the host filesystem can expose privileged system credentials (such as for the kubelet) or privileged APIs
(such as the container runtime socket) that can be used for container escape or to attack other
parts of the cluster.
* Pods with identical configuration (such as created from a PodTemplate) may
behave differently on different nodes due to different files on the nodes.
* `hostPath` volume usage is not treated as ephemeral storage usage.
You need to monitor the disk usage by yourself because excessive `hostPath` disk
usage will lead to disk pressure on the node.
Some uses for a `hostPath` are:
* running a container that needs access to node-level system components
(such as a container that transfers system logs to a central location,
accessing those logs using a read-only mount of `/var/log`)
* making a configuration file stored on the host system available read-only
to a [static Pod](/docs/tasks/configure-pod-container/static-pod/);
unlike normal Pods, static Pods cannot access ConfigMaps#### `hostPath` volume types
In addition to the required `path` property, you can optionally specify a
`type` for a `hostPath` volume.
The available values for `type` are:
|Value|Behavior|
|`‌""`|Empty string (default) is for backward compatibility, which means that no checks will be performed before mounting the `hostPath` volume.|
|`DirectoryOrCreate`|If nothing exists at the given path, an empty directory will be created there as needed with permission set to 0755, having the same group and ownership with Kubelet.|
|`Directory`|A directory must exist at the given path.|
|`FileOrCreate`|If nothing exists at the given path, an empty file will be created there as needed with permission set to 0644, having the same group and ownership with Kubelet.|
|`File`|A file must exist at the given path.|
|`Socket`|A UNIX socket must exist at the given path.|
|`CharDevice`|*(Linux nodes only)* A character device must exist at the given path.|
|`BlockDevice`|*(Linux nodes only)* A block device must exist at the given path.|