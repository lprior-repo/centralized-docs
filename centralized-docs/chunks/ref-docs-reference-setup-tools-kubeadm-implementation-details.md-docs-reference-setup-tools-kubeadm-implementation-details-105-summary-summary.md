---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#105-summary
chunk_level: summary
chunk_type: prose
heading: kubeadm upgrade workflow internal design
token_count: 122
summary: * Performs any post-upgrade tasks, such as, cleaning up deprecated features which are release specific.### kubeadm upgrade node `kubeadm upgrade node` upgrades a single control plane or worker node...
---

* Performs any post-upgrade tasks, such as, cleaning up deprecated features which are release specific.### kubeadm upgrade node
`kubeadm upgrade node` upgrades a single control plane or worker node after the cluster upgrade has
started (by running `kubeadm upgrade apply`). The command detects if the node is a control plane node by checking
if the file `/etc/kubernetes/manifests/kube-apiserver.yaml` exists. On finding that file, the kubeadm tool
infers that there is a running kube-apiserver Pod on this node.