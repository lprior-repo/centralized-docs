---
doc_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability
chunk_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability#6-detailed
chunk_level: detailed
chunk_type: prose
heading: External etcd nodes
token_count: 899
summary: ## External etcd nodes Setting up a cluster with external etcd nodes is similar to the procedure used for stacked etcd with the exception that you should setup etcd first, and you should pass the...
---

## External etcd nodes
Setting up a cluster with external etcd nodes is similar to the procedure used for stacked etcd
with the exception that you should setup etcd first, and you should pass the etcd information
in the kubeadm config file.
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
#### Note:
The difference between stacked etcd and external etcd here is that the external etcd setup requires
a configuration file with the etcd endpoints under the `external` object for `etcd`.
In the case of the stacked etcd topology, this is managed automatically.
* Replace the following variables in the config template with the appropriate values for your cluster:
* `LOAD\_BALANCER\_DNS`
* `LOAD\_BALANCER\_PORT`
* `ETCD\_0\_IP`
* `ETCD\_1\_IP`
* `ETCD\_2\_IP`
The following steps are similar to the stacked etcd setup:
1. Run `sudo kubeadm init --config kubeadm-config.yaml --upload-certs` on this node.
2. Write the output join commands that are returned to a text file for later use.
3. Apply the CNI plugin of your choice.
#### Note:
You must pick a network plugin that suits your use case and deploy it before you move on to next step.
If you don't do this, you will not be able to launch your cluster properly.
### Steps for the rest of the control plane nodes
The steps are the same as for the stacked etcd setup:
* Make sure the first control plane node is fully initialized.
* Join each control plane node with the join command you saved to a text file. It's recommended
to join the control plane nodes one at a time.
* Don't forget that the decryption key from `--certificate-key` expires after two hours, by default.## Common tasks after bootstrapping control plane
### Install workers
Worker nodes can be joined to the cluster with the command you stored previously
as the output from the `kubeadm init` command:
```
`sudo kubeadm join 192.168.0.200:6443 --token 9vr73a.a8uxyaju799qwdjv --discovery-token-ca-cert-hash sha256:7c2e69131a36ae2a042a339b33381c6d0d43887e2de83720eff5359e26aec866
`
```