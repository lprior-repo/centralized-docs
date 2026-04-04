---
doc_id: tutorial/docs-tasks-administer-cluster-sysctl-cluster.md/docs-tasks-administer-cluster-sysctl-cluster
chunk_id: tutorial/docs-tasks-administer-cluster-sysctl-cluster.md/docs-tasks-administer-cluster-sysctl-cluster#3-detailed
chunk_level: detailed
chunk_type: prose
heading: Related Pages
token_count: 951
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
#### Warning:
Due to their nature of being *unsafe*, the use of *unsafe* sysctls
is at-your-own-risk and can lead to severe problems like wrong behavior of
containers, resource shortage or complete breakage of a node.
It is good practice to consider nodes with special sysctl settings as
*tainted* within a cluster, and only schedule pods onto them which need those
sysctl settings. It is suggested to use the Kubernetes [*taints and toleration*
feature](/docs/reference/generated/kubectl/kubectl-commands/#taint) to implement this.
A pod with the *unsafe* sysctls will fail to launch on any node which has not
enabled those two *unsafe* sysctls explicitly. As with *node-level* sysctls it
is recommended to use
[*taints and toleration* feature](/docs/reference/generated/kubectl/kubectl-commands/#taint) or
[taints on nodes](/docs/concepts/scheduling-eviction/taint-and-toleration/)
to schedule those pods onto the right nodes.
## Feedback
Was this page helpful?
Yes
No
Thanks for the feedback. If you have a specific, answerable question about how to use Kubernetes, ask it on
[Stack Overflow](https://stackoverflow.com/questions/tagged/kubernetes).
Open an issue in the [GitHub Repository](https://www.github.com/kubernetes/website/) if you want to
[report a problem](<https://github.com/kubernetes/website/issues/new?title=Issue with k8s.io>)
or
[suggest an improvement](<https://github.com/kubernetes/website/issues/new?title=Improvement for k8s.io>).
Last modified September 20, 2024 at 11:36 AM PST: [sync safe sysctl ipv4.rmen and ipv4.wmem for v1.32 (de6ead9316)](https://github.com/kubernetes/website/commit/de6ead9316ae495ccc84ec728f87459d24f9dd85)
## Related Pages

- [Securing a Cluster](docs-tasks-administer-cluster-securing-a-cluster.md)
- [Binding](docs-reference-kubernetes-api-workload-resources-binding-v1.md)
- [conventions](docs-reference-kubectl-conventions.md)
- [HorizontalPodAutoscaler](docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md)
- [Концепции](ru-docs-concepts.md)