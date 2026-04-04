---
doc_id: ref/docs-reference-kubernetes-api-cluster-resources-lease-candidate-v1beta1.md/docs-reference-kubernetes-api-cluster-resources-lease-candidate-v1beta1
chunk_id: ref/docs-reference-kubernetes-api-cluster-resources-lease-candidate-v1beta1.md/docs-reference-kubernetes-api-cluster-resources-lease-candidate-v1beta1#12-summary
chunk_level: summary
chunk_type: prose
heading: LeaseCandidateSpec
token_count: 57
summary: * **emulationVersion** (string) EmulationVersion is the emulation version. It must be in a semver format without leading `v`. EmulationVersion must be less than or equal to BinaryVersion. This field...
---

* **emulationVersion** (string)
EmulationVersion is the emulation version. It must be in a semver format without leading `v`. EmulationVersion must be less than or equal to BinaryVersion. This field is required when strategy is "OldestEmulationVersion"