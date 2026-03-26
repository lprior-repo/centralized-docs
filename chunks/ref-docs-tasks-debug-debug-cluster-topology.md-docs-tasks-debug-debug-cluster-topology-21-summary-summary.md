---
doc_id: ref/docs-tasks-debug-debug-cluster-topology.md/docs-tasks-debug-debug-cluster-topology
chunk_id: ref/docs-tasks-debug-debug-cluster-topology.md/docs-tasks-debug-debug-cluster-topology#21-summary
chunk_level: summary
chunk_type: prose
heading: Check the device plugin resource API
token_count: 97
summary: ## Check the device plugin resource API The kubelet provides a `PodResourceLister` gRPC service to enable discovery of resources and associated metadata. By using its [List gRPC...
---

## Check the device plugin resource API
The kubelet provides a `PodResourceLister` gRPC service to enable discovery of resources and associated metadata.
By using its [List gRPC endpoint](/docs/concepts/extend-kubernetes/compute-storage-net/device-plugins/#grpc-endpoint-list),
information about reserved memory for each container can be retrieved, which is contained
in protobuf `ContainerMemory` message.
This information can be retrieved solely for pods in Guaranteed QoS class.