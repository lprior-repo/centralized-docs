---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#17-summary
chunk_level: summary
chunk_type: prose
heading: Core design principles
token_count: 110
summary: `kind` of configuration file. For example, `InitConfiguration` for `kubeadm init`, `JoinConfiguration` for `kubeadm join`, `UpgradeConfiguration` for `kubeadm upgrade` and `ResetConfiguration` for...
---

`kind` of configuration file.
For example, `InitConfiguration` for `kubeadm init`, `JoinConfiguration` for `kubeadm join`, `UpgradeConfiguration` for `kubeadm upgrade` and `ResetConfiguration`
for `kubeadm reset`.
The command `kubeadm config migrate` can be used to migrate an older format configuration
file to a newer (current) configuration format. The kubeadm tool only supports migrating from
deprecated configuration formats to the current format.
See the