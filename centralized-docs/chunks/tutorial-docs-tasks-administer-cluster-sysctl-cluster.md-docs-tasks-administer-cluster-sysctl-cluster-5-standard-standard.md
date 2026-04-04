---
doc_id: tutorial/docs-tasks-administer-cluster-sysctl-cluster.md/docs-tasks-administer-cluster-sysctl-cluster
chunk_id: tutorial/docs-tasks-administer-cluster-sysctl-cluster.md/docs-tasks-administer-cluster-sysctl-cluster#5-standard
chunk_level: standard
chunk_type: prose
heading: Setting Sysctls for a Pod
token_count: 429
summary: ## Setting Sysctls for a Pod A number of sysctls are *namespaced* in today's Linux kernels. This means that they can be set independently for each pod on a node. Only namespaced sysctls are...
---

## Setting Sysctls for a Pod
A number of sysctls are *namespaced* in today's Linux kernels. This means that
they can be set independently for each pod on a node. Only namespaced sysctls
are configurable via the pod securityContext within Kubernetes.
The following sysctls are known to be namespaced. This list could change
in future versions of the Linux kernel.
* `kernel.shm\*`,
* `kernel.msg\*`,
* `kernel.sem`,
* `fs.mqueue.\*`,
* Those `net.\*` that can be set in container networking namespace. However,
there are exceptions (e.g., `net.netfilter.nf\_conntrack\_max` and
`net.netfilter.nf\_conntrack\_expect\_max` can be set in container networking
namespace but are unnamespaced before Linux 5.12.2).
Sysctls with no namespace are called *node-level* sysctls. If you need to set
them, you must manually configure them on each node's operating system, or by
using a DaemonSet with privileged containers.
Use the pod securityContext to configure namespaced sysctls. The securityContext
applies to all containers in the same pod.
This example uses the pod securityContext to set a safe sysctl
`kernel.shm\_rmid\_forced` and two unsafe sysctls `net.core.somaxconn` and
`kernel.msgmax`. There is no distinction between *safe* and *unsafe* sysctls in
the specification.
#### Warning:
Only modify sysctl parameters after you understand their effects, to avoid
destabilizing your operating system.
```
`apiVersion: v1
kind: Pod
metadata:
name: sysctl-example
spec:
securityContext:
sysctls:
- name: kernel.shm\_rmid\_forced
value: "0"
- name: net.core.somaxconn
value: "1024"
- name: kernel.msgmax
value: "65536"
...
`
```