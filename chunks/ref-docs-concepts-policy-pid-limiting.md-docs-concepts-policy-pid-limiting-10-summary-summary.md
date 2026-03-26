---
doc_id: ref/docs-concepts-policy-pid-limiting.md/docs-concepts-policy-pid-limiting
chunk_id: ref/docs-concepts-policy-pid-limiting.md/docs-concepts-policy-pid-limiting#10-summary
chunk_level: summary
chunk_type: prose
heading: Pod PID limits
token_count: 108
summary: ## Pod PID limits Kubernetes allows you to limit the number of processes running in a Pod. You specify this limit at the node level, rather than configuring it as a resource limit for a particular...
---

## Pod PID limits
Kubernetes allows you to limit the number of processes running in a Pod. You
specify this limit at the node level, rather than configuring it as a resource
limit for a particular Pod. Each Node can have a different PID limit.
To configure the limit, you can specify the command line parameter `--pod-max-pids`
to the kubelet, or set `PodPidsLimit` in the kubelet
[configuration file](/docs/tasks/administer-cluster/kubelet-config-file/).