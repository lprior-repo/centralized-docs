---
doc_id: ref/docs-tasks-debug-debug-cluster-topology.md/docs-tasks-debug-debug-cluster-topology
chunk_id: ref/docs-tasks-debug-debug-cluster-topology.md/docs-tasks-debug-debug-cluster-topology#2-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 126
summary: Kubernetes keeps many aspects of how pods execute on nodes abstracted from the user. This is by design. However, some workloads require stronger guarantees in terms of latency and/or performance in...
---

Kubernetes keeps many aspects of how pods execute on nodes abstracted
from the user. This is by design. However, some workloads require
stronger guarantees in terms of latency and/or performance in order to operate
acceptably. The `kubelet` provides methods to enable more complex workload
placement policies while keeping the abstraction free from explicit placement
directives.
You can manage topology within nodes. This means helping the kubelet to configure the host operating system so that
Pods and containers are placed on the correct side of inner boundaries, such as *NUMA domains*. (NUMA is an abbreviation
of