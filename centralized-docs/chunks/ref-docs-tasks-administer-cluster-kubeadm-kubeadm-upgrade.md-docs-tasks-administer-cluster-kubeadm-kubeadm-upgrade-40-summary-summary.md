---
doc_id: ref/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade.md/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade
chunk_id: ref/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade.md/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade#40-summary
chunk_level: summary
chunk_type: prose
heading: Recovering from a failure state
token_count: 115
summary: ## Recovering from a failure state If `kubeadm upgrade` fails and does not roll back, for example because of an unexpected shutdown during execution, you can run `kubeadm upgrade` again. This command...
---

## Recovering from a failure state
If `kubeadm upgrade` fails and does not roll back, for example because of an unexpected shutdown during execution, you can run `kubeadm upgrade` again.
This command is idempotent and eventually makes sure that the actual state is the desired state you declare.
To recover from a bad state, you can also run `sudo kubeadm upgrade apply --force` without changing the version that your cluster is running.
During upgrade kubeadm writes the following backup folders under `/etc/kubernetes/tmp`: