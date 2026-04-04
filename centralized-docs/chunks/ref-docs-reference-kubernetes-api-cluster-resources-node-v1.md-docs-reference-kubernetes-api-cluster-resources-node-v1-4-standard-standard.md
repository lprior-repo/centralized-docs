---
doc_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1
chunk_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1#4-standard
chunk_level: standard
chunk_type: prose
heading: NodeSpec
token_count: 498
summary: * **externalID** (string) Deprecated. Not all kubelets will set this field. Remove field after 1.13. see: [https://issues.k8s.io/61966](https://issues.k8s.io/61966) * **podCIDR** (string) PodCIDR...
---

* **externalID** (string)
Deprecated. Not all kubelets will set this field. Remove field after 1.13. see: [https://issues.k8s.io/61966](https://issues.k8s.io/61966)
* **podCIDR** (string)
PodCIDR represents the pod IP range assigned to the node.
* **podCIDRs** ([]string)
*Set: unique values will be kept during a merge*
podCIDRs represents the IP ranges assigned to the node for usage by Pods on that node. If this field is specified, the 0th entry must match the podCIDR field. It may contain at most 1 value for each of IPv4 and IPv6.
* **providerID** (string)
ID of the node assigned by the cloud provider in the format: &lt;ProviderName&gt;://&lt;ProviderSpecificNodeID&gt;
* **taints** ([]Taint)
*Atomic: will be replaced during a merge*
If specified, the node's taints.
*The node this Taint is attached to has the "effect" on any pod that does not tolerate the Taint.*
* **taints.effect** (string), required
Required. The effect of the taint on pods that do not tolerate the taint. Valid effects are NoSchedule, PreferNoSchedule and NoExecute.
Possible enum values:
* `"NoExecute"` Evict any already-running pods that do not tolerate the taint. Currently enforced by NodeController.
* `"NoSchedule"` Do not allow new pods to schedule onto the node unless they tolerate the taint, but allow all pods submitted to Kubelet without going through the scheduler to start, and allow all already-running pods to continue running. Enforced by the scheduler.
* `"PreferNoSchedule"` Like TaintEffectNoSchedule, but the scheduler tries not to schedule new pods onto the node, rather than prohibiting new pods from scheduling onto the node entirely. Enforced by the scheduler.
* **taints.key** (string), required
Required. The taint key to be applied to a node.
* **taints.timeAdded** (Time)
TimeAdded represents the time at which the taint was added.
*Time is a wrapper around time.Time which supports correct marshaling to YAML and JSON. Wrappers are provided for many of the factory methods that the time package offers.*