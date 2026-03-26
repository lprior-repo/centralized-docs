---
doc_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm
chunk_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm#6-standard
chunk_level: standard
chunk_type: prose
heading: Objectives
token_count: 356
summary: ### Initializing your control-plane node The control-plane node is the machine where the control plane components run, including [etcd](/docs/tasks/administer-cluster/configure-upgrade-etcd/) (the...
---

### Initializing your control-plane node
The control-plane node is the machine where the control plane components run, including
[etcd](/docs/tasks/administer-cluster/configure-upgrade-etcd/) (the cluster database) and the
[API Server](/docs/concepts/architecture/#kube-apiserver)
(which the [kubectl](/docs/reference/kubectl/) command line tool
communicates with).
1. (Recommended) If you have plans to upgrade this single control-plane `kubeadm` cluster
to [high availability](/docs/setup/production-environment/tools/kubeadm/high-availability/)
you should specify the `--control-plane-endpoint` to set the shared endpoint for all control-plane nodes.
Such an endpoint can be either a DNS name or an IP address of a load-balancer.
2. Choose a Pod network add-on, and verify whether it requires any arguments to
be passed to `kubeadm init`. Depending on which
third-party provider you choose, you might need to set the `--pod-network-cidr` to
a provider-specific value. See [Installing a Pod network add-on](#pod-network).
3. (Optional) `kubeadm` tries to detect the container runtime by using a list of well
known endpoints. To use different container runtime or if there are more than one installed
on the provisioned node, specify the `--cri-socket` argument to `kubeadm`. See
[Installing a runtime](/docs/setup/production-environment/tools/kubeadm/install-kubeadm/#installing-runtime).
To initialize the control-plane node run:
```
`kubeadm init &lt;args&gt;
`
```