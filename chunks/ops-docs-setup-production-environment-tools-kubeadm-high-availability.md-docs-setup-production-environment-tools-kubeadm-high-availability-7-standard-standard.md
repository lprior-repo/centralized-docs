---
doc_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability
chunk_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability#7-standard
chunk_level: standard
chunk_type: prose
heading: Before you begin
token_count: 509
summary: #### Note: Some CNI network plugins require additional configuration, for example specifying the pod IP CIDR, while others do not. See the [CNI network...
---

#### Note:
Some CNI network plugins require additional configuration, for example specifying the pod IP CIDR, while others do not.
See the [CNI network documentation](/docs/setup/production-environment/tools/kubeadm/create-cluster-kubeadm/#pod-network).
To add a pod CIDR pass the flag `--pod-network-cidr`, or if you are using a kubeadm configuration file
set the `podSubnet` field under the `networking` object of `ClusterConfiguration`.
The output looks similar to:
```
`...
You can now join any number of control-plane node by running the following command on each as a root:
kubeadm join 192.168.0.200:6443 --token 9vr73a.a8uxyaju799qwdjv --discovery-token-ca-cert-hash sha256:7c2e69131a36ae2a042a339b33381c6d0d43887e2de83720eff5359e26aec866 --control-plane --certificate-key f8902e114ef118304e561c3ecd4d0b543adc226b7a07f675f56564185ffe0c07
Please note that the certificate-key gives access to cluster sensitive data, keep it secret!
As a safeguard, uploaded-certs will be deleted in two hours; If necessary, you can use kubeadm init phase upload-certs to reload certs afterward.
Then you can join any number of worker nodes by running the following on each as root:
kubeadm join 192.168.0.200:6443 --token 9vr73a.a8uxyaju799qwdjv --discovery-token-ca-cert-hash sha256:7c2e69131a36ae2a042a339b33381c6d0d43887e2de83720eff5359e26aec866
`
```
* Copy this output to a text file. You will need it later to join control plane and worker nodes to
the cluster.
* When `--upload-certs` is used with `kubeadm init`, the certificates of the primary control plane
are encrypted and uploaded in the `kubeadm-certs` Secret.
* To re-upload the certificates and generate a new decryption key, use the following command on a
control plane
node that is already joined to the cluster: