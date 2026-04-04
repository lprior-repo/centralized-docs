---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#49-summary
chunk_level: summary
chunk_type: prose
heading: kubeadm init workflow internal design
token_count: 76
summary: 1. All images will be pulled from registry.k8s.io by default. See [using custom images](/docs/reference/setup-tools/kubeadm/kubeadm-init/#custom-images) for customizing the image repository 2. In...
---

1. All images will be pulled from registry.k8s.io by default.
See [using custom images](/docs/reference/setup-tools/kubeadm/kubeadm-init/#custom-images)
for customizing the image repository
2. In case kubeadm is executed in the `--dry-run` mode, static Pod files are written in a
temporary folder