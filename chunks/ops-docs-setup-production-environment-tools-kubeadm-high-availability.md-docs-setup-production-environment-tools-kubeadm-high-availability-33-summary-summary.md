---
doc_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability
chunk_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability#33-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 127
summary: Then you can join any number of worker nodes by running the following on each as root: kubeadm join 192.168.0.200:6443 --token 9vr73a.a8uxyaju799qwdjv --discovery-token-ca-cert-hash...
---

Then you can join any number of worker nodes by running the following on each as root:
kubeadm join 192.168.0.200:6443 --token 9vr73a.a8uxyaju799qwdjv --discovery-token-ca-cert-hash sha256:7c2e69131a36ae2a042a339b33381c6d0d43887e2de83720eff5359e26aec866
`
```
* Copy this output to a text file. You will need it later to join control plane and worker nodes to
the cluster.