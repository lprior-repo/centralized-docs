---
doc_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm
chunk_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm#47-summary
chunk_level: summary
chunk_type: prose
heading: Objectives
token_count: 87
summary: #### Caution: This section contains important information about networking setup and deployment order. Read all of this advice carefully before proceeding. **You must deploy a [Container Network...
---

#### Caution:
This section contains important information about networking setup and
deployment order.
Read all of this advice carefully before proceeding.
**You must deploy a
[Container Network Interface](/docs/concepts/extend-kubernetes/compute-storage-net/network-plugins/)
(CNI) based Pod network add-on so that your Pods can communicate with each other.
Cluster DNS (CoreDNS) will not start up before a network is installed.**