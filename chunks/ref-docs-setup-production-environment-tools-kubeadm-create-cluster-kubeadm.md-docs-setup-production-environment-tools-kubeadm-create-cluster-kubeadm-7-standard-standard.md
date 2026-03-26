---
doc_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm
chunk_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm#7-standard
chunk_level: standard
chunk_type: prose
heading: Objectives
token_count: 253
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
Where `192.168.0.102` is the IP address of this node and `cluster-endpoint` is a custom DNS name that maps to this IP.
This will allow you to pass `--control-plane-endpoint=cluster-endpoint` to `kubeadm init` and pass the same DNS name to
`kubeadm join`. Later you can modify `cluster-endpoint` to point to the address of your load-balancer in a
high availability scenario.
Turning a single control plane cluster created without `--control-plane-endpoint` into a highly available cluster
is not supported by kubeadm.