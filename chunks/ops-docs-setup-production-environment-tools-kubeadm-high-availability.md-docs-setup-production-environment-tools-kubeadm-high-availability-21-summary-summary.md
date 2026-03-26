---
doc_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability
chunk_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability#21-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 121
summary: * In a cloud environment you should place your control plane nodes behind a TCP forwarding load balancer. This load balancer distributes traffic to all healthy control plane nodes in its target list....
---

* In a cloud environment you should place your control plane nodes behind a TCP
forwarding load balancer. This load balancer distributes traffic to all
healthy control plane nodes in its target list. The health check for
an apiserver is a TCP check on the port the kube-apiserver listens on
(default value `:6443`).
* It is not recommended to use an IP address directly in a cloud environment.
* The load balancer must be able to communicate with all control plane nodes
on the apiserver port. It must also allow incoming traffic on its
listening port.