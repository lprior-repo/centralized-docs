---
doc_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability
chunk_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability#46-summary
chunk_level: summary
chunk_type: prose
heading: External etcd nodes
token_count: 108
summary: `--- apiVersion: kubeadm.k8s.io/v1beta4 kind: ClusterConfiguration kubernetesVersion: stable controlPlaneEndpoint: \"LOAD\_BALANCER\_DNS:LOAD\_BALANCER\_PORT\" # change this (see below) etcd: external:...
---

`---
apiVersion: kubeadm.k8s.io/v1beta4
kind: ClusterConfiguration
kubernetesVersion: stable
controlPlaneEndpoint: "LOAD\_BALANCER\_DNS:LOAD\_BALANCER\_PORT" # change this (see below)
etcd:
external:
endpoints:
- https://ETCD\_0\_IP:2379 # change ETCD\_0\_IP appropriately
- https://ETCD\_1\_IP:2379 # change ETCD\_1\_IP appropriately