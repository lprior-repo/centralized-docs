---
doc_id: ref/docs-concepts-policy-pid-limiting.md/docs-concepts-policy-pid-limiting
chunk_id: ref/docs-concepts-policy-pid-limiting.md/docs-concepts-policy-pid-limiting#1-standard
chunk_level: standard
chunk_type: prose
heading: Table of Contents
token_count: 239
summary: # Process ID Limits And Reservations FEATURE STATE: `Kubernetes v1.20 [stable]` Kubernetes allow you to limit the number of process IDs (PIDs) that a [Pod](/docs/concepts/workloads/pods/) can use....
---

# Process ID Limits And Reservations
FEATURE STATE:
`Kubernetes v1.20 [stable]`
Kubernetes allow you to limit the number of process IDs (PIDs) that a
[Pod](/docs/concepts/workloads/pods/) can use.
You can also reserve a number of allocatable PIDs for each [node](/docs/concepts/architecture/nodes/)
for use by the operating system and daemons (rather than by Pods).
Process IDs (PIDs) are a fundamental resource on nodes. It is trivial to hit the
task limit without hitting any other resource limits, which can then cause
instability to a host machine.
Cluster administrators require mechanisms to ensure that Pods running in the
cluster cannot induce PID exhaustion that prevents host daemons (such as the
[kubelet](/docs/reference/command-line-tools-reference/kubelet) or
[kube-proxy](/docs/reference/command-line-tools-reference/kube-proxy/),
and potentially also the container runtime) from running.
In addition, it is important to ensure that PIDs are limited among Pods in order
to ensure they have limited impact on other workloads on the same node.