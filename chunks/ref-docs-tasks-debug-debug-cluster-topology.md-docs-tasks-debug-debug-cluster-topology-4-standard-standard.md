---
doc_id: ref/docs-tasks-debug-debug-cluster-topology.md/docs-tasks-debug-debug-cluster-topology
chunk_id: ref/docs-tasks-debug-debug-cluster-topology.md/docs-tasks-debug-debug-cluster-topology#4-standard
chunk_level: standard
chunk_type: prose
heading: Check the device plugin resource API
token_count: 388
summary: It can be deduced from the state file that the pod was pinned to both NUMA nodes, i.e.: ``` `\"numaAffinity\":[ 0, 1 ], ` ``` Pinned term means that pod's memory consumption is constrained (through...
---

It can be deduced from the state file that the pod was pinned to both NUMA nodes, i.e.:
```
`"numaAffinity":[
0,
1
],
`
```
Pinned term means that pod's memory consumption is constrained (through `cgroups` configuration)
to these NUMA nodes.
This automatically implies that Memory Manager instantiated a new group that
comprises these two NUMA nodes, i.e. `0` and `1` indexed NUMA nodes.
In order to analyse memory resources available in a group,the corresponding entries from
NUMA nodes belonging to the group must be added up.
For example, the total amount of free "conventional" memory in the group can be computed
by adding up the free memory available at every NUMA node in the group,
i.e., in the `"memory"` section of NUMA node `0` (`"free":0`) and NUMA node `1` (`"free":103739236352`).
So, the total amount of free "conventional" memory in this group is equal to `0 + 103739236352` bytes.
The line `"systemReserved":3221225472` indicates that the administrator of this node reserved
`3221225472` bytes (i.e. `3Gi`) to serve kubelet and system processes at NUMA node `0`,
by using `--reserved-memory` flag.
## Check the device plugin resource API
The kubelet provides a `PodResourceLister` gRPC service to enable discovery of resources and associated metadata.
By using its [List gRPC endpoint](/docs/concepts/extend-kubernetes/compute-storage-net/device-plugins/#grpc-endpoint-list),
information about reserved memory for each container can be retrieved, which is contained
in protobuf `ContainerMemory` message.
This information can be retrieved solely for pods in Guaranteed QoS class.