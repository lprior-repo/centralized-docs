---
doc_id: tutorial/docs-tasks-administer-cluster-memory-manager.md/docs-tasks-administer-cluster-memory-manager
chunk_id: tutorial/docs-tasks-administer-cluster-memory-manager.md/docs-tasks-administer-cluster-memory-manager#2-standard
chunk_level: standard
chunk_type: prose
heading: Before you begin
token_count: 384
summary: ## Before you begin You need to have a Kubernetes cluster, and the kubectl command-line tool must be configured to communicate with your cluster. It is recommended to run this tutorial on a cluster...
---

## Before you begin
You need to have a Kubernetes cluster, and the kubectl command-line tool must
be configured to communicate with your cluster. It is recommended to run this tutorial on a cluster with at least two nodes that are not acting as control plane hosts. If you do not already have a
cluster, you can create one by using
[minikube](https://minikube.sigs.k8s.io/docs/tutorials/multi_node/)
or you can use one of these Kubernetes playgrounds:
* [iximiuz Labs](https://labs.iximiuz.com/playgrounds?category=kubernetes&amp;filter=all)
* [Killercoda](https://killercoda.com/playgrounds/scenario/kubernetes)
* [KodeKloud](https://kodekloud.com/public-playgrounds)
Your Kubernetes server must be at or later than version v1.32.
To check the version, enter `kubectl version`.
If you are running an older version of Kubernetes, check the documentation
for the version of Kubernetes you are running.
### Resource alignment prerequisites
To align memory resources with other requested resources in a Pod spec:
* the CPU Manager should be enabled and proper CPU Manager policy should be configured on a Node.
See [control CPU Management Policies](/docs/tasks/administer-cluster/cpu-management-policies/);
* the Topology Manager should be enabled and proper Topology Manager policy should be configured on a Node.
See [control Topology Management Policies](/docs/tasks/administer-cluster/topology-manager/).### Windows support
FEATURE STATE:
`Kubernetes v1.32 [alpha]`(disabled by default)
Windows support can be enabled via the `WindowsCPUAndMemoryAffinity` feature gate
and it requires support in the container runtime.
Only the [None](#policy-none) and [BestEffort](#policy-best-effort) policies are supported on Windows.