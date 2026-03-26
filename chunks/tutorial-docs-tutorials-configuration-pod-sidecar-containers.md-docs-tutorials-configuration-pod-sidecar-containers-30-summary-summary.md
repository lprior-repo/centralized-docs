---
doc_id: tutorial/docs-tutorials-configuration-pod-sidecar-containers.md/docs-tutorials-configuration-pod-sidecar-containers
chunk_id: tutorial/docs-tutorials-configuration-pod-sidecar-containers.md/docs-tutorials-configuration-pod-sidecar-containers#30-summary
chunk_level: summary
chunk_type: prose
heading: Adopting built-in sidecar containers
token_count: 60
summary: 1. Mark Pods that land to nodes supporting sidecars. You can use node labels and node affinity to mark nodes supporting sidecar containers and Pods landing on those nodes. 2. Check Nodes...
---

1. Mark Pods that land to nodes supporting sidecars. You can use node labels
and node affinity to mark nodes supporting sidecar containers and Pods landing on those nodes.
2. Check Nodes compatibility on injection. During sidecar injection, you may use
the following strategies to check node compatibility: