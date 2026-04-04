---
doc_id: tutorial/docs-tasks-administer-cluster-securing-a-cluster.md/docs-tasks-administer-cluster-securing-a-cluster
chunk_id: tutorial/docs-tasks-administer-cluster-securing-a-cluster.md/docs-tasks-administer-cluster-securing-a-cluster#8-standard
chunk_level: standard
chunk_type: prose
heading: Controlling the capabilities of a workload or user at runtime
token_count: 302
summary: ### Restricting cloud metadata API access Cloud platforms (AWS, Azure, GCE, etc.) often expose metadata services locally to instances. By default these APIs are accessible by pods running on an...
---

### Restricting cloud metadata API access
Cloud platforms (AWS, Azure, GCE, etc.) often expose metadata services locally to instances.
By default these APIs are accessible by pods running on an instance and can contain cloud
credentials for that node, or provisioning data such as kubelet credentials. These credentials
can be used to escalate within the cluster or to other cloud services under the same account.
When running Kubernetes on a cloud platform, limit permissions given to instance credentials, use
[network policies](/docs/tasks/administer-cluster/declare-network-policy/) to restrict pod access
to the metadata API, and avoid using provisioning data to deliver secrets.
### Controlling which nodes pods may access
By default, there are no restrictions on which nodes may run a pod. Kubernetes offers a
[rich set of policies for controlling placement of pods onto nodes](/docs/concepts/scheduling-eviction/assign-pod-node/)
and the [taint-based pod placement and eviction](/docs/concepts/scheduling-eviction/taint-and-toleration/)
that are available to end users. For many clusters use of these policies to separate workloads
can be a convention that authors adopt or enforce via tooling.
As an administrator, a beta admission plugin `PodNodeSelector` can be used to force pods
within a namespace to default or require a specific node selector, and if end users cannot
alter namespaces, this can strongly limit the placement of all of the pods in a specific workload.