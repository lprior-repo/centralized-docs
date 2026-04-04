---
doc_id: ref/docs-reference-setup-tools-kubeadm.md/docs-reference-setup-tools-kubeadm
chunk_id: ref/docs-reference-setup-tools-kubeadm.md/docs-reference-setup-tools-kubeadm#1-standard
chunk_level: standard
chunk_type: prose
heading: How to install
token_count: 200
summary: # Kubeadm ![](/images/kubeadm-stacked-color.png)Kubeadm is a tool built to provide `kubeadm init` and `kubeadm join` as best-practice \"fast paths\" for creating Kubernetes clusters. kubeadm performs...
---

# Kubeadm
![](/images/kubeadm-stacked-color.png)Kubeadm is a tool built to provide `kubeadm init` and `kubeadm join` as best-practice "fast paths" for creating Kubernetes clusters.
kubeadm performs the actions necessary to get a minimum viable cluster up and running. By design, it cares only about bootstrapping, not about provisioning machines. Likewise, installing various nice-to-have addons, like the Kubernetes Dashboard, monitoring solutions, and cloud-specific addons, is not in scope.
Instead, we expect higher-level and more tailored tooling to be built on top of kubeadm, and ideally, using kubeadm as the basis of all deployments will make it easier to create conformant clusters.
## How to install
To install kubeadm, see the [installation guide](/docs/setup/production-environment/tools/kubeadm/install-kubeadm/).