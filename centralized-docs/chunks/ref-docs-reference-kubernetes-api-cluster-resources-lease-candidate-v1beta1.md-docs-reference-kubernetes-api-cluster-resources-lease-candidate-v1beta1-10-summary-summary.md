---
doc_id: ref/docs-reference-kubernetes-api-cluster-resources-lease-candidate-v1beta1.md/docs-reference-kubernetes-api-cluster-resources-lease-candidate-v1beta1
chunk_id: ref/docs-reference-kubernetes-api-cluster-resources-lease-candidate-v1beta1.md/docs-reference-kubernetes-api-cluster-resources-lease-candidate-v1beta1#10-summary
chunk_level: summary
chunk_type: prose
heading: LeaseCandidateSpec
token_count: 90
summary: * **binaryVersion** (string), required BinaryVersion is the binary version. It must be in a semver format without leading `v`. This field is required. * **leaseName** (string), required LeaseName is...
---

* **binaryVersion** (string), required
BinaryVersion is the binary version. It must be in a semver format without leading `v`. This field is required.
* **leaseName** (string), required
LeaseName is the name of the lease for which this candidate is contending. The limits on this field are the same as on Lease.name. Multiple lease candidates may reference the same Lease.name. This field is immutable.