---
doc_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm
chunk_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm#5-detailed
chunk_level: detailed
chunk_type: prose
heading: Objectives
token_count: 879
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
#### Note:
Kubeadm should be CNI agnostic and the validation of CNI providers is out of the scope of our current e2e testing.
If you find an issue related to a CNI plugin you should log a ticket in its respective issue
tracker instead of the kubeadm or kubernetes issue trackers.
Several external projects provide Kubernetes Pod networks using CNI, some of which also
support [Network Policy](/docs/concepts/services-networking/network-policies/).
See a list of add-ons that implement the
[Kubernetes networking model](/docs/concepts/cluster-administration/networking/#how-to-implement-the-kubernetes-network-model).
Please refer to the [Installing Addons](/docs/concepts/cluster-administration/addons/#networking-and-network-policy)
page for a non-exhaustive list of networking addons supported by Kubernetes.
You can install a Pod network add-on with the following command on the
control-plane node or a node that has the kubeconfig credentials:
```
`kubectl apply -f &lt;add-on.yaml&gt;
`
```
#### Note:
Only a few CNI plugins support Windows. More details and setup instructions can be found
in [Adding Windows worker nodes](/docs/tasks/administer-cluster/kubeadm/adding-windows-nodes/#network-config).
You can install only one Pod network per cluster.
Once a Pod network has been installed, you can confirm that it is working by
checking that the CoreDNS Pod is `Running` in the output of `kubectl get pods --all-namespaces`.
And once the CoreDNS Pod is up and running, you can continue by joining your nodes.
If your network is not working or CoreDNS is not in the `Running` state, check out the
[troubleshooting guide](/docs/setup/production-environment/tools/kubeadm/troubleshooting-kubeadm/)
for `kubeadm`.
### Managed node labels
By default, kubeadm enables the [NodeRestriction](/docs/reference/access-authn-authz/admission-controllers/#noderestriction)
admission controller that restricts what labels can be self-applied by kubelets on node registration.
The admission controller documentation covers what labels are permitted to be used with the kubelet `--node-labels` option.
The `node-role.kubernetes.io/control-plane` label is such a restricted label and kubeadm manually applies it using
a privileged client after a node has been created. To do that manually you can do the same by using `kubectl label`
and ensure it is using a privileged kubeconfig such as the kubeadm managed `/etc/kubernetes/admin.conf`.