---
doc_id: ref/docs-tasks-debug-debug-cluster-topology.md/docs-tasks-debug-debug-cluster-topology
chunk_id: ref/docs-tasks-debug-debug-cluster-topology.md/docs-tasks-debug-debug-cluster-topology#3-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 72
summary: Pods and containers are placed on the correct side of inner boundaries, such as *NUMA domains*. (NUMA is an abbreviation of *non-uniform memory access*, and refers to an idea that CPUs might be...
---

Pods and containers are placed on the correct side of inner boundaries, such as *NUMA domains*. (NUMA is an abbreviation
of *non-uniform memory access*, and refers to an idea that CPUs might be topologically closer to specific regions of
memory, due to the physical layout of the hardware components and the way that these are connected).