---
doc_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1
chunk_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1#18-summary
chunk_level: summary
chunk_type: prose
heading: NodeSpec
token_count: 127
summary: * **taints.effect** (string), required Required. The effect of the taint on pods that do not tolerate the taint. Valid effects are NoSchedule, PreferNoSchedule and NoExecute. Possible enum values: *...
---

* **taints.effect** (string), required
Required. The effect of the taint on pods that do not tolerate the taint. Valid effects are NoSchedule, PreferNoSchedule and NoExecute.
Possible enum values:
* `"NoExecute"` Evict any already-running pods that do not tolerate the taint. Currently enforced by NodeController.
* `"NoSchedule"` Do not allow new pods to schedule onto the node unless they tolerate the taint, but allow all pods submitted to Kubelet without going through the scheduler to start, and allow all already-running pods to continue running. Enforced by the scheduler.