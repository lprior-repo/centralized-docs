---
doc_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability
chunk_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability#53-summary
chunk_level: summary
chunk_type: prose
heading: External etcd nodes
token_count: 119
summary: ### Install workers Worker nodes can be joined to the cluster with the command you stored previously as the output from the `kubeadm init` command: ``` `sudo kubeadm join 192.168.0.200:6443 --token...
---

### Install workers
Worker nodes can be joined to the cluster with the command you stored previously
as the output from the `kubeadm init` command:
```
`sudo kubeadm join 192.168.0.200:6443 --token 9vr73a.a8uxyaju799qwdjv --discovery-token-ca-cert-hash sha256:7c2e69131a36ae2a042a339b33381c6d0d43887e2de83720eff5359e26aec866
`
```