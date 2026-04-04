---
doc_id: tutorial/docs-tasks-administer-cluster-memory-manager.md/docs-tasks-administer-cluster-memory-manager
chunk_id: tutorial/docs-tasks-administer-cluster-memory-manager.md/docs-tasks-administer-cluster-memory-manager#1-detailed
chunk_level: detailed
chunk_type: prose
heading: How does the Memory Manager operate?
token_count: 943
summary: # Control Memory Management Policies on a Node FEATURE STATE: `Kubernetes v1.32 [stable]`(enabled by default) The Kubernetes *Memory Manager* enables the feature of guaranteed memory (and hugepages)...
---

# Control Memory Management Policies on a Node
FEATURE STATE:
`Kubernetes v1.32 [stable]`(enabled by default)
The Kubernetes *Memory Manager* enables the feature of guaranteed memory (and hugepages)
allocation for pods in the `Guaranteed` [QoS class](/docs/concepts/workloads/pods/pod-qos/).
The Memory Manager employs a hint generation protocol to yield the most suitable NUMA affinity for a pod.
The Memory Manager feeds the central manager (*Topology Manager*) with these affinity hints.
Based on both the hints and Topology Manager policy, the pod is rejected or admitted to the node.
Moreover, the Memory Manager ensures that the memory which a pod requests
is allocated from a minimum number of NUMA nodes.
For background about memory resources for Pods, read
[Assign Memory Resources to Containers and Pods](/docs/tasks/configure-pod-container/assign-memory-resource/).
## Before you begin
You need to have a Kubernetes cluster, and the kubectl command-line tool must
be configured to communicate with your cluster. It is recommended to run this tutorial on a cluster with at least two nodes that are not acting as control plane hosts. If you do not already have a
cluster, you can create one by using
[minikube](https://minikube.sigs.k8s.io/docs/tutorials/multi_node/)
or you can use one of these Kubernetes playgrounds:
* [iximiuz Labs](https://labs.iximiuz.com/playgrounds?category=kubernetes&amp;filter=all)
* [Killercoda](https://killercoda.com/playgrounds/scenario/kubernetes)
* [KodeKloud](https://kodekloud.com/public-playgrounds)
Your Kubernetes server must be at or later than version v1.32.
To check the version, enter `kubectl version`.
If you are running an older version of Kubernetes, check the documentation
for the version of Kubernetes you are running.
### Resource alignment prerequisites
To align memory resources with other requested resources in a Pod spec:
* the CPU Manager should be enabled and proper CPU Manager policy should be configured on a Node.
See [control CPU Management Policies](/docs/tasks/administer-cluster/cpu-management-policies/);
* the Topology Manager should be enabled and proper Topology Manager policy should be configured on a Node.
See [control Topology Management Policies](/docs/tasks/administer-cluster/topology-manager/).### Windows support
FEATURE STATE:
`Kubernetes v1.32 [alpha]`(disabled by default)
Windows support can be enabled via the `WindowsCPUAndMemoryAffinity` feature gate
and it requires support in the container runtime.
Only the [None](#policy-none) and [BestEffort](#policy-best-effort) policies are supported on Windows.
## How does the Memory Manager operate?
For Linux nodes, the Memory Manager offers the guaranteed memory (and hugepages) allocation
for Pods in Guaranteed QoS class.
To immediately put the Memory Manager into operation follow the guidelines in the section
[Memory Manager configuration](#memory-manager-configuration), and subsequently,
prepare and deploy a `Guaranteed` Pod as illustrated in the section
[Placing a Pod in the Guaranteed QoS class](#placing-a-pod-in-the-guaranteed-qos-class).
The Memory Manager is a hint provider, and it provides topology hints for
the Topology Manager which then aligns the requested resources according to these topology hints.
On Linux, it also enforces `cgroups` (specifically, `cpuset.mems`) for Pods.
The complete flow diagram concerning pod admission and deployment process is illustrated
below:
![Memory Manager in the pod admission and deployment process](/images/docs/memory-manager-diagram.svg)
During this process, the Memory Manager updates its internal counters stored in
[Node Map and Memory Maps][2] to manage guaranteed memory allocation.
The memory manager activates during kubelet startup if a node administrator configures
`reservedMemory` for the kubelet (section [Reserved memory configuration](#reserved-memory-flag)).
In this case, the kubelet updates its node map to reflect this reservation.
When the `Static` policy is configured, you **must** configure reserved memory for the node
(for example, with the `reservedMemory` configuration field in the kubelet configuration).
An important topic in the context of Memory Manager operation is the management of NUMA groups.
Each time pod's memory request is in excess of single NUMA node capacity, the Memory Manager
attempts to create a group that comprises several NUMA nodes and that features extended memory
capacity.