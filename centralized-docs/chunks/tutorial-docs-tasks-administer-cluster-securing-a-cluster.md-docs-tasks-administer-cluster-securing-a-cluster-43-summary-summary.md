---
doc_id: tutorial/docs-tasks-administer-cluster-securing-a-cluster.md/docs-tasks-administer-cluster-securing-a-cluster
chunk_id: tutorial/docs-tasks-administer-cluster-securing-a-cluster.md/docs-tasks-administer-cluster-securing-a-cluster#43-summary
chunk_level: summary
chunk_type: prose
heading: Controlling the capabilities of a workload or user at runtime
token_count: 123
summary: By default, there are no restrictions on which nodes may run a pod. Kubernetes offers a [rich set of policies for controlling placement of pods onto...
---

By default, there are no restrictions on which nodes may run a pod. Kubernetes offers a
[rich set of policies for controlling placement of pods onto nodes](/docs/concepts/scheduling-eviction/assign-pod-node/)
and the [taint-based pod placement and eviction](/docs/concepts/scheduling-eviction/taint-and-toleration/)
that are available to end users. For many clusters use of these policies to separate workloads
can be a convention that authors adopt or enforce via tooling.
As an administrator, a beta admission plugin `PodNodeSelector` can be used to force pods