---
doc_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm
chunk_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm#69-summary
chunk_level: summary
chunk_type: prose
heading: Objectives
token_count: 67
summary: this with the `kubeadm kubeconfig user --client-name &lt;CN&gt;` command. That command will print out a KubeConfig file to STDOUT which you should save to a file and distribute to your user. After...
---

this with the `kubeadm kubeconfig user --client-name &lt;CN&gt;`
command. That command will print out a KubeConfig file to STDOUT which you
should save to a file and distribute to your user. After that, grant
privileges by using `kubectl create (cluster)rolebinding`.