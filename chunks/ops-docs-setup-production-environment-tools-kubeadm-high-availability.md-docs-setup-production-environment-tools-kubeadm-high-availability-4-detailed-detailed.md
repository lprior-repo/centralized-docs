---
doc_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability
chunk_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability#4-detailed
chunk_level: detailed
chunk_type: prose
heading: Before you begin
token_count: 994
summary: ### Steps for the first control plane node 1. Initialize the control plane: ``` `sudo kubeadm init --control-plane-endpoint \"LOAD\_BALANCER\_DNS:LOAD\_BALANCER\_PORT\" --upload-certs ` ``` * You can...
---

### Steps for the first control plane node
1. Initialize the control plane:
```
`sudo kubeadm init --control-plane-endpoint "LOAD\_BALANCER\_DNS:LOAD\_BALANCER\_PORT" --upload-certs
`
```
* You can use the `--kubernetes-version` flag to set the Kubernetes version to use.
It is recommended that the versions of kubeadm, kubelet, kubectl and Kubernetes match.
* The `--control-plane-endpoint` flag should be set to the address or DNS and port of the load balancer.
* The `--upload-certs` flag is used to upload the certificates that should be shared
across all the control-plane instances to the cluster. If instead, you prefer to copy certs across
control-plane nodes manually or using automation tools, please remove this flag and refer to [Manual
certificate distribution](#manual-certs) section below.
#### Note:
The `kubeadm init` flags `--config` and `--certificate-key` cannot be mixed, therefore if you want
to use the [kubeadm configuration](/docs/reference/config-api/kubeadm-config.v1beta4/)
you must add the `certificateKey` field in the appropriate config locations
(under `InitConfiguration` and `JoinConfiguration: controlPlane`).
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
```
`sudo kubeadm init phase upload-certs --upload-certs
`
```
* You can also specify a custom `--certificate-key` during `init` that can later be used by `join`.
To generate such a key you can use the following command:
```
`kubeadm certs certificate-key
`
```
The certificate key is a hex encoded string that is an AES key of size 32 bytes.
#### Note:
The `kubeadm-certs` Secret and the decryption key expire after two hours.
#### Caution:
As stated in the command output, the certificate key gives access to cluster sensitive data, keep it secret!
* Apply the CNI plugin of your choice:
[Follow these instructions](/docs/setup/production-environment/tools/kubeadm/create-cluster-kubeadm/#pod-network)
to install the CNI provider. Make sure the configuration corresponds to the Pod CIDR specified in the
kubeadm configuration file (if applicable).