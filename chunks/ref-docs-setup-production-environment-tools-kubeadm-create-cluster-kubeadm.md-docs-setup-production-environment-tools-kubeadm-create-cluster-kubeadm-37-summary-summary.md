---
doc_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm
chunk_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm#37-summary
chunk_level: summary
chunk_type: prose
heading: Objectives
token_count: 96
summary: . If you join a node with a different architecture to your cluster, make sure that your deployed DaemonSets have container image support for this architecture. `kubeadm init` first runs a series of...
---

.
If you join a node with a different architecture to your cluster, make sure that your deployed DaemonSets
have container image support for this architecture.
`kubeadm init` first runs a series of prechecks to ensure that the machine
is ready to run Kubernetes. These prechecks expose warnings and exit on errors. `kubeadm init`
then downloads and installs the cluster control plane components. This may take several minutes.
After it finishes you should see: