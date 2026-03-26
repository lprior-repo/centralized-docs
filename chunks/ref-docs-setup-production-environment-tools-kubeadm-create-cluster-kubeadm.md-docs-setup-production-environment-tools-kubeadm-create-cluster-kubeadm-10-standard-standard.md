---
doc_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm
chunk_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm#10-standard
chunk_level: standard
chunk_type: prose
heading: Objectives
token_count: 329
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
* Take care that your Pod network must not overlap with any of the host
networks: you are likely to see problems if there is any overlap.
(If you find a collision between your network plugin's preferred Pod
network and some of your host networks, you should think of a suitable
CIDR block to use instead, then use that during `kubeadm init` with
`--pod-network-cidr` and as a replacement in your network plugin's YAML).
* By default, `kubeadm` sets up your cluster to use and enforce use of
[RBAC](/docs/reference/access-authn-authz/rbac/) (role based access
control).
Make sure that your Pod network plugin supports RBAC, and so do any manifests
that you use to deploy it.
* If you want to use IPv6--either dual-stack, or single-stack IPv6 only
networking--for your cluster, make sure that your Pod network plugin
supports IPv6.
IPv6 support was added to CNI in [v0.6.0](https://github.com/containernetworking/cni/releases/tag/v0.6.0).