---
doc_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm
chunk_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm#31-summary
chunk_level: summary
chunk_type: prose
heading: Objectives
token_count: 122
summary: ### Considerations about apiserver-advertise-address and ControlPlaneEndpoint While `--apiserver-advertise-address` can be used to set the advertised address for this particular control-plane node's...
---

### Considerations about apiserver-advertise-address and ControlPlaneEndpoint
While `--apiserver-advertise-address` can be used to set the advertised address for this particular
control-plane node's API server, `--control-plane-endpoint` can be used to set the shared endpoint
for all control-plane nodes.
`--control-plane-endpoint` allows both IP addresses and DNS names that can map to IP addresses.
Please contact your network administrator to evaluate possible solutions with respect to such mapping.
Here is an example mapping:
```
`192.168.0.102 cluster-endpoint
`
```