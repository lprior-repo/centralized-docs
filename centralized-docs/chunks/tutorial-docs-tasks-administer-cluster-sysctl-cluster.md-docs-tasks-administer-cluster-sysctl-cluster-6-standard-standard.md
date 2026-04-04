---
doc_id: tutorial/docs-tasks-administer-cluster-sysctl-cluster.md/docs-tasks-administer-cluster-sysctl-cluster
chunk_id: tutorial/docs-tasks-administer-cluster-sysctl-cluster.md/docs-tasks-administer-cluster-sysctl-cluster#6-standard
chunk_level: standard
chunk_type: prose
heading: Feedback
token_count: 425
summary: #### Warning: Due to their nature of being *unsafe*, the use of *unsafe* sysctls is at-your-own-risk and can lead to severe problems like wrong behavior of containers, resource shortage or complete...
---

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