---
doc_id: tutorial/docs-tasks-administer-cluster-reserve-compute-resources.md/docs-tasks-administer-cluster-reserve-compute-resources
chunk_id: tutorial/docs-tasks-administer-cluster-reserve-compute-resources.md/docs-tasks-administer-cluster-reserve-compute-resources#32-summary
chunk_level: summary
chunk_type: prose
heading: Node Allocatable
token_count: 114
summary: **KubeletConfiguration Setting**: `evictionHard: {memory.available: \"100Mi\", nodefs.available: \"10%\", nodefs.inodesFree: \"5%\", imagefs.available: \"15%\"}`. Example value: `{memory.available:...
---

**KubeletConfiguration Setting**: `evictionHard: {memory.available: "100Mi", nodefs.available: "10%", nodefs.inodesFree: "5%", imagefs.available: "15%"}`. Example value: `{memory.available: "&lt;500Mi"}`
Memory pressure at the node level leads to System OOMs which affects the entire
node and all pods running on it. Nodes can go offline temporarily until memory
has been reclaimed. To avoid (or reduce the probability of) system OOMs kubelet
provides