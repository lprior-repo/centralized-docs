---
doc_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability
chunk_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability#3-detailed
chunk_level: detailed
chunk_type: prose
heading: Before you begin
token_count: 495
summary: ### Container images Each host should have access read and fetch images from the Kubernetes container image registry, `registry.k8s.io`. If you want to deploy a highly-available cluster where the...
---

### Container images
Each host should have access read and fetch images from the Kubernetes container image registry,
`registry.k8s.io`. If you want to deploy a highly-available cluster where the hosts do not have
access to pull images, this is possible. You must ensure by some other means that the correct
container images are already available on the relevant hosts.
### Command line interface
To manage Kubernetes once your cluster is set up, you should
[install kubectl](/docs/tasks/tools/#kubectl) on your PC. It is also useful
to install the `kubectl` tool on each control plane node, as this can be
helpful for troubleshooting.
#### Note:
There are many configurations for load balancers. The following example is only one
option. Your cluster requirements may need a different configuration.
1. Create a kube-apiserver load balancer with a name that resolves to DNS.
* In a cloud environment you should place your control plane nodes behind a TCP
forwarding load balancer. This load balancer distributes traffic to all
healthy control plane nodes in its target list. The health check for
an apiserver is a TCP check on the port the kube-apiserver listens on
(default value `:6443`).
* It is not recommended to use an IP address directly in a cloud environment.
* The load balancer must be able to communicate with all control plane nodes
on the apiserver port. It must also allow incoming traffic on its
listening port.
* Make sure the address of the load balancer always matches
the address of kubeadm's `ControlPlaneEndpoint`.
* Read the [Options for Software Load Balancing](https://git.k8s.io/kubeadm/docs/ha-considerations.md#options-for-software-load-balancing)
guide for more details.
* Add the first control plane node to the load balancer, and test the
connection:
```
`nc -zv -w 2 &lt;LOAD\_BALANCER\_IP&gt; &lt;PORT&gt;
`
```
A connection refused error is expected because the API server is not yet
running. A timeout, however, means the load balancer cannot communicate
with the control plane node. If a timeout occurs, reconfigure the load
balancer to communicate with the control plane node.
* Add the remaining control plane nodes to the load balancer target group.