---
doc_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm
chunk_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm#3-standard
chunk_level: standard
chunk_type: prose
heading: Objectives
token_count: 394
summary: ## Objectives * Install a single control-plane Kubernetes cluster * Install a Pod network on the cluster so that your Pods can talk to each other## Instructions #### Component installation Install a...
---

## Objectives
* Install a single control-plane Kubernetes cluster
* Install a Pod network on the cluster so that your Pods can
talk to each other## Instructions
#### Component installation
Install a [container runtime](/docs/setup/production-environment/container-runtimes)
and kubeadm on all the hosts. For detailed instructions and other prerequisites, see
[Installing kubeadm](/docs/setup/production-environment/tools/kubeadm/install-kubeadm/).
#### Note:
If you have already installed kubeadm, see the first two steps of the
[Upgrading Linux nodes](/docs/tasks/administer-cluster/kubeadm/upgrading-linux-nodes/)
document for instructions on how to upgrade kubeadm.
When you upgrade, the kubelet restarts every few seconds as it waits in a crashloop for
kubeadm to tell it what to do. This crashloop is expected and normal.
After you initialize your control-plane, the kubelet runs normally.
#### Network setup
kubeadm similarly to other Kubernetes components tries to find a usable IP on
the network interfaces associated with a default gateway on a host. Such
an IP is then used for the advertising and/or listening performed by a component.
To find out what this IP is on a Linux host you can use:
```
`ip route show # Look for a line starting with "default via"
`
```
#### Note:
If two or more default gateways are present on the host, a Kubernetes component will
try to use the first one it encounters that has a suitable global unicast IP address.
While making this choice, the exact ordering of gateways might vary between different
operating systems and kernel versions.
Kubernetes components do not accept custom network interface as an option,
therefore a custom IP address must be passed as a flag to all components instances
that need such a custom configuration.