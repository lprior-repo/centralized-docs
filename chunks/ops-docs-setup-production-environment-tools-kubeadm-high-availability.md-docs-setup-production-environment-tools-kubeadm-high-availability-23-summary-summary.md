---
doc_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability
chunk_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability#23-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 70
summary: A connection refused error is expected because the API server is not yet running. A timeout, however, means the load balancer cannot communicate with the control plane node. If a timeout occurs,...
---

A connection refused error is expected because the API server is not yet
running. A timeout, however, means the load balancer cannot communicate
with the control plane node. If a timeout occurs, reconfigure the load
balancer to communicate with the control plane node.
* Add the remaining control plane nodes to the load balancer target group.