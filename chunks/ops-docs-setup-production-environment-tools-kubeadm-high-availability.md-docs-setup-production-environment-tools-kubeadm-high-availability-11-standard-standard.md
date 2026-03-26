---
doc_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability
chunk_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability#11-standard
chunk_level: standard
chunk_type: prose
heading: External etcd nodes
token_count: 392
summary: ### Set up the etcd cluster 1. Follow these [instructions](/docs/setup/production-environment/tools/kubeadm/setup-ha-etcd-with-kubeadm/) to set up the etcd cluster. 2. Set up SSH as described...
---

### Set up the etcd cluster
1. Follow these [instructions](/docs/setup/production-environment/tools/kubeadm/setup-ha-etcd-with-kubeadm/) to set up the etcd cluster.
2. Set up SSH as described [here](#manual-certs).
3. Copy the following files from any etcd node in the cluster to the first control plane node:
```
`export CONTROL\_PLANE="ubuntu@10.0.0.7"
scp /etc/kubernetes/pki/etcd/ca.crt "${CONTROL\_PLANE}":
scp /etc/kubernetes/pki/apiserver-etcd-client.crt "${CONTROL\_PLANE}":
scp /etc/kubernetes/pki/apiserver-etcd-client.key "${CONTROL\_PLANE}":
`
```
* Replace the value of `CONTROL\_PLANE` with the `user@host` of the first control-plane node.### Set up the first control plane node
1. Create a file called `kubeadm-config.yaml` with the following contents:
```
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
- https://ETCD\_2\_IP:2379 # change ETCD\_2\_IP appropriately
caFile: /etc/kubernetes/pki/etcd/ca.crt
certFile: /etc/kubernetes/pki/apiserver-etcd-client.crt
keyFile: /etc/kubernetes/pki/apiserver-etcd-client.key
`
```