---
doc_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm
chunk_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm#48-summary
chunk_level: summary
chunk_type: prose
heading: Objectives
token_count: 99
summary: * Take care that your Pod network must not overlap with any of the host networks: you are likely to see problems if there is any overlap. (If you find a collision between your network plugin's...
---

* Take care that your Pod network must not overlap with any of the host
networks: you are likely to see problems if there is any overlap.
(If you find a collision between your network plugin's preferred Pod
network and some of your host networks, you should think of a suitable
CIDR block to use instead, then use that during `kubeadm init` with
`--pod-network-cidr` and as a replacement in your network plugin's YAML).