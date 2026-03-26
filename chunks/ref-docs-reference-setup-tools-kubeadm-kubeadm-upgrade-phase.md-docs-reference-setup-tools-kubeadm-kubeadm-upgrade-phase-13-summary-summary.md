---
doc_id: ref/docs-reference-setup-tools-kubeadm-kubeadm-upgrade-phase.md/docs-reference-setup-tools-kubeadm-kubeadm-upgrade-phase
chunk_id: ref/docs-reference-setup-tools-kubeadm-kubeadm-upgrade-phase.md/docs-reference-setup-tools-kubeadm-kubeadm-upgrade-phase#13-summary
chunk_level: summary
chunk_type: table
heading: kubeadm upgrade apply phase
token_count: 124
summary: '. Value 'all' ignores errors from all checks. | |--kubeconfig stringDefault: \"/etc/kubernetes/admin.conf\"| || The kubeconfig file to use when talking to the cluster. If the flag is not set, a set of...
---

'. Value 'all' ignores errors from all checks.
|
|--kubeconfig stringDefault: "/etc/kubernetes/admin.conf"|
||
The kubeconfig file to use when talking to the cluster. If the flag is not set, a set of standard locations can be searched for an existing kubeconfig file.
|
|-y, --yes|
||
Perform the upgrade and do not prompt for confirmation (non-interactive mode).
|
### Options inherited from parent commands
|--rootfs string|
||
The path to the 'real' host root filesystem. This will cause kubeadm to chroot into the provided path.
|