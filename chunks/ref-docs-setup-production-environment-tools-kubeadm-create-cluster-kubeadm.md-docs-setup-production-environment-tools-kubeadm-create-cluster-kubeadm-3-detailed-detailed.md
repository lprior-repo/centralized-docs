---
doc_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm
chunk_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm#3-detailed
chunk_level: detailed
chunk_type: prose
heading: Objectives
token_count: 773
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
### Considerations about apiserver-advertise-address and ControlPlaneEndpoint
While `--apiserver-advertise-address` can be used to set the advertised address for this particular
control-plane node's API server, `--control-plane-endpoint` can be used to set the shared endpoint
for all control-plane nodes.
`--control-plane-endpoint` allows both IP addresses and DNS names that can map to IP addresses.
Please contact your network administrator to evaluate possible solutions with respect to such mapping.
Here is an example mapping:
```
`192.168.0.102 cluster-endpoint
`
```
Where `192.168.0.102` is the IP address of this node and `cluster-endpoint` is a custom DNS name that maps to this IP.
This will allow you to pass `--control-plane-endpoint=cluster-endpoint` to `kubeadm init` and pass the same DNS name to
`kubeadm join`. Later you can modify `cluster-endpoint` to point to the address of your load-balancer in a
high availability scenario.
Turning a single control plane cluster created without `--control-plane-endpoint` into a highly available cluster
is not supported by kubeadm.