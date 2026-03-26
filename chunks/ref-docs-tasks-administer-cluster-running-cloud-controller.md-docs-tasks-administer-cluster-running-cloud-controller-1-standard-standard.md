---
doc_id: ref/docs-tasks-administer-cluster-running-cloud-controller.md/docs-tasks-administer-cluster-running-cloud-controller
chunk_id: ref/docs-tasks-administer-cluster-running-cloud-controller.md/docs-tasks-administer-cluster-running-cloud-controller#1-standard
chunk_level: standard
chunk_type: prose
heading: Table of Contents
token_count: 175
summary: # Cloud Controller Manager Administration FEATURE STATE: `Kubernetes v1.11 [beta]` Since cloud providers develop and release at a different pace compared to the Kubernetes project, abstracting the...
---

# Cloud Controller Manager Administration
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
provided in the core Kubernetes project uses the same cloud libraries as `kube-controller-manager`.
Cloud providers already supported in Kubernetes core are expected to use the in-tree
cloud-controller-manager to transition out of Kubernetes core.