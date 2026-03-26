---
doc_id: ref/docs-concepts-policy-pid-limiting.md/docs-concepts-policy-pid-limiting
chunk_id: ref/docs-concepts-policy-pid-limiting.md/docs-concepts-policy-pid-limiting#9-summary
chunk_level: summary
chunk_type: prose
heading: Node PID limits
token_count: 95
summary: ## Node PID limits Kubernetes allows you to reserve a number of process IDs for the system use. To configure the reservation, use the parameter `pid=&lt;number&gt;` in the `--system-reserved` and...
---

## Node PID limits
Kubernetes allows you to reserve a number of process IDs for the system use. To
configure the reservation, use the parameter `pid=&lt;number&gt;` in the
`--system-reserved` and `--kube-reserved` command line options to the kubelet.
The value you specified declares that the specified number of process IDs will
be reserved for the system as a whole and for Kubernetes system daemons
respectively.