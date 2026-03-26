---
doc_id: ref/docs-tasks-debug-debug-cluster-topology.md/docs-tasks-debug-debug-cluster-topology
chunk_id: ref/docs-tasks-debug-debug-cluster-topology.md/docs-tasks-debug-debug-cluster-topology#5-standard
chunk_level: standard
chunk_type: prose
heading: Related Pages
token_count: 419
summary: ## Check the device plugin resource API The kubelet provides a `PodResourceLister` gRPC service to enable discovery of resources and associated metadata. By using its [List gRPC...
---

## Check the device plugin resource API
The kubelet provides a `PodResourceLister` gRPC service to enable discovery of resources and associated metadata.
By using its [List gRPC endpoint](/docs/concepts/extend-kubernetes/compute-storage-net/device-plugins/#grpc-endpoint-list),
information about reserved memory for each container can be retrieved, which is contained
in protobuf `ContainerMemory` message.
This information can be retrieved solely for pods in Guaranteed QoS class.
## Feedback
Was this page helpful?
Yes
No
Thanks for the feedback. If you have a specific, answerable question about how to use Kubernetes, ask it on
[Stack Overflow](https://stackoverflow.com/questions/tagged/kubernetes).
Open an issue in the [GitHub Repository](https://www.github.com/kubernetes/website/) if you want to
[report a problem](<https://github.com/kubernetes/website/issues/new?title=Issue with k8s.io>)
or
[suggest an improvement](<https://github.com/kubernetes/website/issues/new?title=Improvement for k8s.io>).
Last modified April 01, 2025 at 2:55 PM PST: [Improve docs for memory manager (fa812c0368)](https://github.com/kubernetes/website/commit/fa812c036873cf7cb25c36cbcaf07ece35c0147b)
## Related Pages

- [Adding entries to Pod /etc/hosts with HostAliases](docs-tasks-network-customize-hosts-file-for-pods.md)
- [Change the Access Mode of a PersistentVolume to ReadWriteOncePod](docs-tasks-administer-cluster-change-pv-access-mode-readwriteoncepod.md)
- [Example: Deploying Cassandra with a StatefulSet](docs-tutorials-stateful-application-cassandra.md)
- [Configure Quality of Service for Pods](docs-tasks-configure-pod-container-quality-service-pod.md)
- [Configure Certificate Rotation for the Kubelet](docs-tasks-tls-certificate-rotation.md)