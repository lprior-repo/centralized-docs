---
doc_id: ref/docs-reference-setup-tools-kubeadm.md/docs-reference-setup-tools-kubeadm
chunk_id: ref/docs-reference-setup-tools-kubeadm.md/docs-reference-setup-tools-kubeadm#2-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 112
summary: ![](/images/kubeadm-stacked-color.png)Kubeadm is a tool built to provide `kubeadm init` and `kubeadm join` as best-practice \"fast paths\" for creating Kubernetes clusters. kubeadm performs the actions...
---

![](/images/kubeadm-stacked-color.png)Kubeadm is a tool built to provide `kubeadm init` and `kubeadm join` as best-practice "fast paths" for creating Kubernetes clusters.
kubeadm performs the actions necessary to get a minimum viable cluster up and running. By design, it cares only about bootstrapping, not about provisioning machines. Likewise, installing various nice-to-have addons, like the Kubernetes Dashboard, monitoring solutions, and cloud-specific addons, is not in scope.