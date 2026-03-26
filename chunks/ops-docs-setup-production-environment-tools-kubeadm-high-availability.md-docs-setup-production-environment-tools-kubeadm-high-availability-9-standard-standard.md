---
doc_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability
chunk_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability#9-standard
chunk_level: standard
chunk_type: prose
heading: Before you begin
token_count: 309
summary: ### Steps for the rest of the control plane nodes For each additional control plane node you should: 1. Execute the join command that was previously given to you by the `kubeadm init` output on the...
---

### Steps for the rest of the control plane nodes
For each additional control plane node you should:
1. Execute the join command that was previously given to you by the `kubeadm init` output on the first node.
It should look something like this:
```
`sudo kubeadm join 192.168.0.200:6443 --token 9vr73a.a8uxyaju799qwdjv --discovery-token-ca-cert-hash sha256:7c2e69131a36ae2a042a339b33381c6d0d43887e2de83720eff5359e26aec866 --control-plane --certificate-key f8902e114ef118304e561c3ecd4d0b543adc226b7a07f675f56564185ffe0c07
`
```
* The `--control-plane` flag tells `kubeadm join` to create a new control plane.
* The `--certificate-key ...` will cause the control plane certificates to be downloaded
from the `kubeadm-certs` Secret in the cluster and be decrypted using the given key.
#### Note:
As the cluster nodes are usually initialized sequentially, the CoreDNS Pods are likely to all run
on the first control plane node. To provide higher availability, please rebalance the CoreDNS Pods
with `kubectl -n kube-system rollout restart deployment coredns` after at least one new node is joined.