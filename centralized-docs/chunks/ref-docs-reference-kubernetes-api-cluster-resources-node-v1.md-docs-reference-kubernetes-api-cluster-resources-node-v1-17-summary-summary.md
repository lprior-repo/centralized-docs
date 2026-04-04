---
doc_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1
chunk_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1#17-summary
chunk_level: summary
chunk_type: prose
heading: NodeSpec
token_count: 93
summary: * **providerID** (string) ID of the node assigned by the cloud provider in the format: &lt;ProviderName&gt;://&lt;ProviderSpecificNodeID&gt; * **taints** ([]Taint) *Atomic: will be replaced during a...
---

* **providerID** (string)
ID of the node assigned by the cloud provider in the format: &lt;ProviderName&gt;://&lt;ProviderSpecificNodeID&gt;
* **taints** ([]Taint)
*Atomic: will be replaced during a merge*
If specified, the node's taints.
*The node this Taint is attached to has the "effect" on any pod that does not tolerate the Taint.*