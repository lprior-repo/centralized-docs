---
doc_id: ref/docs-tasks-administer-cluster-running-cloud-controller.md/docs-tasks-administer-cluster-running-cloud-controller
chunk_id: ref/docs-tasks-administer-cluster-running-cloud-controller.md/docs-tasks-administer-cluster-running-cloud-controller#6-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 127
summary: FEATURE STATE: `Kubernetes v1.11 [beta]` Since cloud providers develop and release at a different pace compared to the Kubernetes project, abstracting the provider-specific code to the...
---

FEATURE STATE:
`Kubernetes v1.11 [beta]`
Since cloud providers develop and release at a different pace compared to the
Kubernetes project, abstracting the provider-specific code to the
`[cloud-controller-manager](/docs/concepts/architecture/cloud-controller/)`
binary allows cloud vendors to evolve independently from the core Kubernetes code.
The `cloud-controller-manager` can be linked to any cloud provider that satisfies
[cloudprovider.Interface](https://github.com/kubernetes/cloud-provider/blob/master/cloud.go).
For backwards compatibility, the
[cloud-controller-manager](https://github.com/kubernetes/kubernetes/tree/master/cmd/cloud-controller-manager)