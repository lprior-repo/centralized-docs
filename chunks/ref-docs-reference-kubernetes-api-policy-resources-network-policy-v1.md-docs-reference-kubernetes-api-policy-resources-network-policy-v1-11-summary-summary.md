---
doc_id: ref/docs-reference-kubernetes-api-policy-resources-network-policy-v1.md/docs-reference-kubernetes-api-policy-resources-network-policy-v1
chunk_id: ref/docs-reference-kubernetes-api-policy-resources-network-policy-v1.md/docs-reference-kubernetes-api-policy-resources-network-policy-v1#11-summary
chunk_level: summary
chunk_type: prose
heading: NetworkPolicySpec
token_count: 72
summary: * **ingress** ([]NetworkPolicyIngressRule) *Atomic: will be replaced during a merge* ingress is a list of ingress rules to be applied to the selected pods. Traffic is allowed to a pod if there are no...
---

* **ingress** ([]NetworkPolicyIngressRule)
*Atomic: will be replaced during a merge*
ingress is a list of ingress rules to be applied to the selected pods. Traffic is allowed to a pod if there are no NetworkPolicies selecting the pod (and cluster policy otherwise allows the traffic), OR if the traffic source is the pod'