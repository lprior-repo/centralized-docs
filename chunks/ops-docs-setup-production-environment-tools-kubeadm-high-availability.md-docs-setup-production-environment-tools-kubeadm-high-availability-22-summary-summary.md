---
doc_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability
chunk_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability#22-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 119
summary: * Make sure the address of the load balancer always matches the address of kubeadm's `ControlPlaneEndpoint`. * Read the [Options for Software Load...
---

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