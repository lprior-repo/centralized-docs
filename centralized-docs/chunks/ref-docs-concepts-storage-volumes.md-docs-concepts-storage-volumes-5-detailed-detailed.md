---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#5-detailed
chunk_level: detailed
chunk_type: table
heading: Types of volumes
token_count: 1004
summary: ### hostPath A `hostPath` volume mounts a file or directory from the host node's filesystem into your Pod. This is not something that most Pods will need, but it offers a powerful escape hatch for...
---

### hostPath
A `hostPath` volume mounts a file or directory from the host node's filesystem
into your Pod. This is not something that most Pods will need, but it offers a
powerful escape hatch for some applications.
#### Warning:
Using the `hostPath` volume type presents many security risks.
If you can avoid using a `hostPath` volume, you should. For example,
define a [`local` PersistentVolume](#local), and use that instead.
If you are restricting access to specific directories on the node using
admission-time validation, that restriction is only effective when you
additionally require that any mounts of that `hostPath` volume are
**read only**. If you allow a read-write mount of any host path by an
untrusted Pod, the containers in that Pod may be able to subvert the
read-write host mount.
Take care when using `hostPath` volumes, whether these are mounted as read-only
or as read-write, because:
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
#### Caution:
The `FileOrCreate` mode does **not** create the parent directory of the file. If the parent directory
of the mounted file does not exist, the Pod fails to start. To ensure that this mode works,
you can try to mount directories and files separately, as shown in the
[`FileOrCreate` example](#hostpath-fileorcreate-example) for `hostPath`.
Some files or directories created on the underlying hosts might only be
accessible by root. You then either need to run your process as root in a
[privileged container](/docs/tasks/configure-pod-container/security-context/)
or modify the file permissions on the host to read from or write to a `hostPath` volume.
# This manifest mounts /data/foo on the host as /foo inside the
# The mount into the container is read-only.
apiVersion: v1
kind: Pod
metadata:
name: hostpath-example-linux
spec:
os: { name: linux }
nodeSelector:
kubernetes.io/os: linux
containers:
- name: example-container
image: registry.k8s.io/test-webserver
volumeMounts:
- mountPath: /foo
name: example-volume
readOnly: true
volumes:
- name: example-volume
# mount /data/foo, but only if that directory already exists
hostPath:
path: /data/foo # directory location on host
type: Directory # this field is optional
`
```
```
`
---
# This manifest mounts C:\\Data\\foo on the host as C:\\foo, inside the