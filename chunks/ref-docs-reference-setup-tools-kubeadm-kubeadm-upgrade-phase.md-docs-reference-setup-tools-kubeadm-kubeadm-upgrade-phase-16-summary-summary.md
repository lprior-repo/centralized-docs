---
doc_id: ref/docs-reference-setup-tools-kubeadm-kubeadm-upgrade-phase.md/docs-reference-setup-tools-kubeadm-kubeadm-upgrade-phase
chunk_id: ref/docs-reference-setup-tools-kubeadm-kubeadm-upgrade-phase.md/docs-reference-setup-tools-kubeadm-kubeadm-upgrade-phase#16-summary
chunk_level: summary
chunk_type: table
heading: kubeadm upgrade apply phase
token_count: 104
summary: |--certificate-renewalDefault: true| || Perform the renewal of certificates used by component changed during upgrades. | |--config string| || Path to a kubeadm configuration file. | |--dry-run| || Do...
---

|--certificate-renewalDefault: true|
||
Perform the renewal of certificates used by component changed during upgrades.
|
|--config string|
||
Path to a kubeadm configuration file.
|
|--dry-run|
||
Do not change any state, just output what actions would be performed.
|
|--etcd-upgradeDefault: true|
||
Perform the upgrade of etcd.
|
|-h, --help|
||
help for control-plane
|
|--kubeconfig stringDefault: "/etc/kubernetes/admin.conf"|
||