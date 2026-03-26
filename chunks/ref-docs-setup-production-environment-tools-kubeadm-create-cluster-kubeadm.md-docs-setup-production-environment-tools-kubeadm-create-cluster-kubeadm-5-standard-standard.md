---
doc_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm
chunk_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm#5-standard
chunk_level: standard
chunk_type: prose
heading: Objectives
token_count: 163
summary: ### Preparing the required container images This step is optional and only applies in case you wish `kubeadm init` and `kubeadm join` to not download the default container images which are hosted at...
---

### Preparing the required container images
This step is optional and only applies in case you wish `kubeadm init` and `kubeadm join`
to not download the default container images which are hosted at `registry.k8s.io`.
Kubeadm has commands that can help you pre-pull the required images
when creating a cluster without an internet connection on its nodes.
See [Running kubeadm without an internet connection](/docs/reference/setup-tools/kubeadm/kubeadm-init/#without-internet-connection)
for more details.
Kubeadm allows you to use a custom image repository for the required images.
See [Using custom images](/docs/reference/setup-tools/kubeadm/kubeadm-init/#custom-images)
for more details.