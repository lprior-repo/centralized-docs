---
doc_id: ref/docs-reference-setup-tools-kubeadm-kubeadm-upgrade-phase.md/docs-reference-setup-tools-kubeadm-kubeadm-upgrade-phase
chunk_id: ref/docs-reference-setup-tools-kubeadm-kubeadm-upgrade-phase.md/docs-reference-setup-tools-kubeadm-kubeadm-upgrade-phase#17-summary
chunk_level: summary
chunk_type: table
heading: kubeadm upgrade apply phase
token_count: 126
summary: | |-h, --help| || help for control-plane | |--kubeconfig stringDefault: \"/etc/kubernetes/admin.conf\"| || The kubeconfig file to use when talking to the cluster. If the flag is not set, a set of...
---

|
|-h, --help|
||
help for control-plane
|
|--kubeconfig stringDefault: "/etc/kubernetes/admin.conf"|
||
The kubeconfig file to use when talking to the cluster. If the flag is not set, a set of standard locations can be searched for an existing kubeconfig file.
|
|--patches string|
||
Path to a directory that contains files named "target[suffix][+patchtype].extension". For example, "kube-apiserver0+merge.yaml" or just "etcd.json". "target" can be one of "kube-apiserver", "kube-controller-manager", "