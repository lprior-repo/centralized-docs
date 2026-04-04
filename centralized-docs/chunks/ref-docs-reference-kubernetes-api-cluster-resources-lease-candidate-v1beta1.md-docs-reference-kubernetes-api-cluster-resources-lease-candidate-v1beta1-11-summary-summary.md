---
doc_id: ref/docs-reference-kubernetes-api-cluster-resources-lease-candidate-v1beta1.md/docs-reference-kubernetes-api-cluster-resources-lease-candidate-v1beta1
chunk_id: ref/docs-reference-kubernetes-api-cluster-resources-lease-candidate-v1beta1.md/docs-reference-kubernetes-api-cluster-resources-lease-candidate-v1beta1#11-summary
chunk_level: summary
chunk_type: prose
heading: LeaseCandidateSpec
token_count: 73
summary: * **strategy** (string), required Strategy is the strategy that coordinated leader election will use for picking the leader. If multiple candidates for the same Lease return different strategies, the...
---

* **strategy** (string), required
Strategy is the strategy that coordinated leader election will use for picking the leader. If multiple candidates for the same Lease return different strategies, the strategy provided by the candidate with the latest BinaryVersion will be used. If there is still conflict, this is a user error and coordinated leader election will not operate the Lease until resolved.