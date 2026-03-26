---
doc_id: ref/docs-tasks-debug-debug-cluster-topology.md/docs-tasks-debug-debug-cluster-topology
chunk_id: ref/docs-tasks-debug-debug-cluster-topology.md/docs-tasks-debug-debug-cluster-topology#1-detailed
chunk_level: detailed
chunk_type: prose
heading: Sources of troubleshooting information
token_count: 457
summary: # Troubleshooting Topology Management Kubernetes keeps many aspects of how pods execute on nodes abstracted from the user. This is by design. However, some workloads require stronger guarantees in...
---

# Troubleshooting Topology Management
Kubernetes keeps many aspects of how pods execute on nodes abstracted
from the user. This is by design. However, some workloads require
stronger guarantees in terms of latency and/or performance in order to operate
acceptably. The `kubelet` provides methods to enable more complex workload
placement policies while keeping the abstraction free from explicit placement
directives.
You can manage topology within nodes. This means helping the kubelet to configure the host operating system so that
Pods and containers are placed on the correct side of inner boundaries, such as *NUMA domains*. (NUMA is an abbreviation
of *non-uniform memory access*, and refers to an idea that CPUs might be topologically closer to specific regions of
memory, due to the physical layout of the hardware components and the way that these are connected).
## Sources of troubleshooting information
You can use the following means to troubleshoot the reason why a pod could not be deployed or
became rejected at a node, in the context of topology management:
* *Pod status* - indicates topology affinity errors
* *system logs* - include valuable information for debugging; for example, about generated hints
* *kubelet state file* - the dump of internal state of the Memory Manager
(including the *node map* and *memory maps*)
* You can use the [device plugin resource API](#device-plugin-resource-api)
to retrieve information about the memory reserved for containers## Troubleshoot `TopologyAffinityError`
This error typically occurs in the following situations:
* a node has not enough resources available to satisfy the pod's request
* the pod's request is rejected due to particular Topology Manager policy constraints
The error appears in the status of a pod:
```
`kubectl get pods
`
```
```
`NAME READY STATUS RESTARTS AGE
guaranteed 0/1 TopologyAffinityError 0 113s
`
```
Use `kubectl describe pod &lt;id&gt;` or `kubectl events` to obtain a detailed error message:
```
`Warning TopologyAffinityError 10m kubelet, dell8 Resources cannot be allocated with Topology locality
`
```