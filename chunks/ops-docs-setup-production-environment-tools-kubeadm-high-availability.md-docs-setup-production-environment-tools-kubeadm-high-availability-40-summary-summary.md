---
doc_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability
chunk_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability#40-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 128
summary: * The `--control-plane` flag tells `kubeadm join` to create a new control plane. * The `--certificate-key ...` will cause the control plane certificates to be downloaded from the `kubeadm-certs`...
---

* The `--control-plane` flag tells `kubeadm join` to create a new control plane.
* The `--certificate-key ...` will cause the control plane certificates to be downloaded
from the `kubeadm-certs` Secret in the cluster and be decrypted using the given key.
#### Note:
As the cluster nodes are usually initialized sequentially, the CoreDNS Pods are likely to all run
on the first control plane node. To provide higher availability, please rebalance the CoreDNS Pods
with `kubectl -n kube-system rollout restart deployment coredns` after at least one new node is joined.