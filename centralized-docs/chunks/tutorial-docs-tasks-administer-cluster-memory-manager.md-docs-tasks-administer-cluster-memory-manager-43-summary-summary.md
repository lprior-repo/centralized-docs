---
doc_id: tutorial/docs-tasks-administer-cluster-memory-manager.md/docs-tasks-administer-cluster-memory-manager
chunk_id: tutorial/docs-tasks-administer-cluster-memory-manager.md/docs-tasks-administer-cluster-memory-manager#43-summary
chunk_level: summary
chunk_type: prose
heading: Reserved memory configuration
token_count: 122
summary: 4. memory type names different than `memory` or `hugepages-&lt;size&gt;` (hugepages of particular `&lt;size&gt;` should also exist).## Placing a Pod in the Guaranteed QoS class If the selected policy...
---

4. memory type names different than `memory` or `hugepages-&lt;size&gt;`
(hugepages of particular `&lt;size&gt;` should also exist).## Placing a Pod in the Guaranteed QoS class
If the selected policy is anything other than `None`, the Memory Manager identifies pods
that are in the `Guaranteed` QoS class.
The Memory Manager provides specific topology hints to the Topology Manager for each `Guaranteed` pod.
For pods in a QoS class other than `Guaranteed`, the Memory Manager provides default topology hints