---
doc_id: ref/docs-reference-setup-tools-kubeadm-kubeadm-upgrade-phase.md/docs-reference-setup-tools-kubeadm-kubeadm-upgrade-phase
chunk_id: ref/docs-reference-setup-tools-kubeadm-kubeadm-upgrade-phase.md/docs-reference-setup-tools-kubeadm-kubeadm-upgrade-phase#40-summary
chunk_level: summary
chunk_type: table
heading: kubeadm upgrade node phase
token_count: 128
summary: |--config string| || Path to a kubeadm configuration file. | |--dry-run| || Do not change any state, just output the actions that would be performed. | |-h, --help| || help for kubelet-config |...
---

|--config string|
||
Path to a kubeadm configuration file.
|
|--dry-run|
||
Do not change any state, just output the actions that would be performed.
|
|-h, --help|
||
help for kubelet-config
|
|--kubeconfig stringDefault: "/etc/kubernetes/admin.conf"|
||
The kubeconfig file to use when talking to the cluster. If the flag is not set, a set of standard locations can be searched for an existing kubeconfig file.
|
|--patches string|
||
Path to a directory that contains files named "target[suffix][+patchtype].extension". For example, "