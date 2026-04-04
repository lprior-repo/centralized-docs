---
doc_id: tutorial/docs-tasks-administer-cluster-sysctl-cluster.md/docs-tasks-administer-cluster-sysctl-cluster
chunk_id: tutorial/docs-tasks-administer-cluster-sysctl-cluster.md/docs-tasks-administer-cluster-sysctl-cluster#23-summary
chunk_level: summary
chunk_type: prose
heading: Setting Sysctls for a Pod
token_count: 117
summary: them, you must manually configure them on each node's operating system, or by using a DaemonSet with privileged containers. Use the pod securityContext to configure namespaced sysctls. The...
---

them, you must manually configure them on each node's operating system, or by
using a DaemonSet with privileged containers.
Use the pod securityContext to configure namespaced sysctls. The securityContext
applies to all containers in the same pod.
This example uses the pod securityContext to set a safe sysctl
`kernel.shm\_rmid\_forced` and two unsafe sysctls `net.core.somaxconn` and
`kernel.msgmax`. There is no distinction between *safe* and *unsafe* sysctls in
the specification.