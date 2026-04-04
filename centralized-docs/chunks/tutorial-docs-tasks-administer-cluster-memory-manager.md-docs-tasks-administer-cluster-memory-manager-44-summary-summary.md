---
doc_id: tutorial/docs-tasks-administer-cluster-memory-manager.md/docs-tasks-administer-cluster-memory-manager
chunk_id: tutorial/docs-tasks-administer-cluster-memory-manager.md/docs-tasks-administer-cluster-memory-manager#44-summary
chunk_level: summary
chunk_type: prose
heading: Reserved memory configuration
token_count: 84
summary: `Guaranteed` pod. For pods in a QoS class other than `Guaranteed`, the Memory Manager provides default topology hints to the Topology Manager. The following excerpts from pod manifests assign a pod...
---

`Guaranteed` pod.
For pods in a QoS class other than `Guaranteed`, the Memory Manager provides default topology hints
to the Topology Manager.
The following excerpts from pod manifests assign a pod to the `Guaranteed` QoS class.
A Pod with integer CPU(s) runs in the `Guaranteed` QoS class, when `requests` are equal to `limits`: