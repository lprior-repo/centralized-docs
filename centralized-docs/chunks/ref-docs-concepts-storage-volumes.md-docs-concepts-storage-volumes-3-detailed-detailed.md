---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#3-detailed
chunk_level: detailed
chunk_type: prose
heading: Types of volumes
token_count: 865
summary: ### downwardAPI A `downwardAPI` volume makes [downward API](/docs/concepts/workloads/pods/downward-api/) data available to applications. Within the volume, you can find the exposed data as read-only...
---

### downwardAPI
A `downwardAPI` volume makes [downward API](/docs/concepts/workloads/pods/downward-api/)
data available to applications. Within the volume, you can find the exposed
data as read-only files in plain text format.
#### Note:
A container using the downward API as a [`subPath`](#using-subpath) volume mount does not
receive updates when field values change.
See [Expose Pod Information to Containers Through Files](/docs/tasks/inject-data-application/downward-api-volume-expose-pod-information/)
to learn more.
### emptyDir
For a Pod that defines an `emptyDir` volume, the volume is created when the Pod is assigned to a node.
As the name says, the `emptyDir` volume is initially empty. All containers in the Pod can read and write the same
files in the `emptyDir` volume, though that volume can be mounted at the same
or different paths in each container. When a Pod is removed from a node for
any reason, the data in the `emptyDir` is deleted permanently.
#### Note:
A container crashing does *not* remove a Pod from a node. The data in an `emptyDir` volume
is safe across container crashes.
Some uses for an `emptyDir` are:
* scratch space, such as for a disk-based merge sort
* checkpointing a long computation for recovery from crashes
* holding files that a content-manager container fetches while a webserver
container serves the data
The `emptyDir.medium` field controls where `emptyDir` volumes are stored. By
default `emptyDir` volumes are stored on whatever medium that backs the node
such as disk, SSD, or network storage, depending on your environment. If you set
the `emptyDir.medium` field to `"Memory"`, Kubernetes mounts a tmpfs (RAM-backed
filesystem) for you instead. While tmpfs is very fast, be aware that, unlike
disks, files you write count against the memory limit of the container that wrote them.
A size limit can be specified for the default medium, which limits the capacity
of the `emptyDir` volume. The storage is allocated from
[node ephemeral storage](/docs/concepts/storage/ephemeral-storage/#setting-requests-and-limits-for-local-ephemeral-storage).
If that is filled up from another source (for example, log files or image overlays),
the `emptyDir` may run out of capacity before this limit.
If no size is specified, memory-backed volumes are sized to node allocatable memory.
#### Caution:
Please check [here](/docs/concepts/configuration/manage-resources-containers/#memory-backed-emptydir)
for points to note in terms of resource management when using memory-backed `emptyDir`.
#### emptyDir configuration example
```
`apiVersion: v1
kind: Pod
metadata:
name: test-pd
spec:
containers:
- image: registry.k8s.io/test-webserver
name: test-container
volumeMounts:
- mountPath: /cache
name: cache-volume
volumes:
- name: cache-volume
emptyDir:
sizeLimit: 500Mi
`
```
#### emptyDir memory configuration example
```
`apiVersion: v1
kind: Pod
metadata:
name: test-pd
spec:
containers:
- image: registry.k8s.io/test-webserver
name: test-container
volumeMounts:
- mountPath: /cache
name: cache-volume
volumes:
- name: cache-volume
emptyDir:
sizeLimit: 500Mi
medium: Memory
`
```
### fc (fibre channel)
An `fc` volume type allows an existing fibre channel block storage volume
to be mounted in a Pod. You can specify single or multiple target world wide names (WWNs)
using the parameter `targetWWNs` in your Volume configuration. If multiple WWNs are specified,
targetWWNs expect that those WWNs are from multi-path connections.
#### Note:
You must configure FC SAN Zoning to allocate and mask those LUNs (volumes) to the target WWNs
beforehand so that Kubernetes hosts can access them.