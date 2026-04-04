---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#108-summary
chunk_level: summary
chunk_type: prose
heading: kubeadm upgrade workflow internal design
token_count: 124
summary: * Performs any post-upgrade tasks, such as cleaning up deprecated features which are release specific.## kubeadm reset workflow internal design You can use the `kubeadm reset` subcommand on a node...
---

* Performs any post-upgrade tasks, such as cleaning up deprecated features which are release specific.## kubeadm reset workflow internal design
You can use the `kubeadm reset` subcommand on a node where kubeadm commands previously executed.
This subcommand performs a **best-effort** cleanup of the node.
If certain actions fail you must intervene and perform manual cleanup.
The command supports phases.
See [`kubeadm reset phase`](/docs/reference/setup-tools/kubeadm/kubeadm-reset-phase/) for more details.
The command supports a configuration file.
Additionally: