---
doc_id: ref/docs-concepts-policy-pid-limiting.md/docs-concepts-policy-pid-limiting
chunk_id: ref/docs-concepts-policy-pid-limiting.md/docs-concepts-policy-pid-limiting#3-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 124
summary: task limit without hitting any other resource limits, which can then cause instability to a host machine. Cluster administrators require mechanisms to ensure that Pods running in the cluster cannot...
---

task limit without hitting any other resource limits, which can then cause
instability to a host machine.
Cluster administrators require mechanisms to ensure that Pods running in the
cluster cannot induce PID exhaustion that prevents host daemons (such as the
[kubelet](/docs/reference/command-line-tools-reference/kubelet) or
[kube-proxy](/docs/reference/command-line-tools-reference/kube-proxy/),
and potentially also the container runtime) from running.
In addition, it is important to ensure that PIDs are limited among Pods in order
to ensure they have limited impact on other workloads on the same node.