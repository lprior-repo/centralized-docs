---
doc_id: ref/docs-reference-kubernetes-api-policy-resources-network-policy-v1.md/docs-reference-kubernetes-api-policy-resources-network-policy-v1
chunk_id: ref/docs-reference-kubernetes-api-policy-resources-network-policy-v1.md/docs-reference-kubernetes-api-policy-resources-network-policy-v1#1-standard
chunk_level: standard
chunk_type: prose
heading: NetworkPolicy
token_count: 207
summary: # NetworkPolicy NetworkPolicy describes what network traffic is allowed for a set of Pods. `apiVersion: networking.k8s.io/v1` `import \"k8s.io/api/networking/v1\"` ## NetworkPolicy NetworkPolicy...
---

# NetworkPolicy
NetworkPolicy describes what network traffic is allowed for a set of Pods.
`apiVersion: networking.k8s.io/v1`
`import "k8s.io/api/networking/v1"`
## NetworkPolicy
NetworkPolicy describes what network traffic is allowed for a set of Pods
* **apiVersion**: networking.k8s.io/v1
* **kind**: NetworkPolicy
* **metadata** ([ObjectMeta](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/object-meta/#ObjectMeta))
Standard object's metadata. More info: [https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata](https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata)
* **spec** ([NetworkPolicySpec](https://kubernetes.io/docs/reference/kubernetes-api/policy-resources/network-policy-v1/#NetworkPolicySpec))
spec represents the specification of the desired behavior for this NetworkPolicy.