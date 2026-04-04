---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#63-summary
chunk_level: summary
chunk_type: prose
heading: kubeadm init workflow internal design
token_count: 83
summary: 1. The etcd container image will be pulled from `registry.gcr.io` by default. See [using custom images](/docs/reference/setup-tools/kubeadm/kubeadm-init/#custom-images) for customizing the image...
---

1. The etcd container image will be pulled from `registry.gcr.io` by default. See
[using custom images](/docs/reference/setup-tools/kubeadm/kubeadm-init/#custom-images)
for customizing the image repository.
2. If you run kubeadm in `--dry-run` mode, the etcd static Pod manifest is written
into a temporary folder.