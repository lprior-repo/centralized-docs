---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#109-summary
chunk_level: summary
chunk_type: prose
heading: kubeadm upgrade workflow internal design
token_count: 112
summary: * IPVS, iptables and nftables rules are **not** cleaned up. * CNI (network plugin) configuration is **not** cleaned up. * `.kube/` in the user's home directory is **not** cleaned up. The command has...
---

* IPVS, iptables and nftables rules are **not** cleaned up.
* CNI (network plugin) configuration is **not** cleaned up.
* `.kube/` in the user's home directory is **not** cleaned up.
The command has the following stages:
* Runs preflight checks on the node to determine if its healthy.
* For control plane nodes, removes any local etcd member data.
* Stops the kubelet.
* Stops running containers.
* Unmounts any mounted directories in `/var/lib/kubelet`.