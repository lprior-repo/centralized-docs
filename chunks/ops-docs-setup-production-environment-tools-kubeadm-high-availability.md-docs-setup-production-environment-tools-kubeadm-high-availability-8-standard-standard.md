---
doc_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability
chunk_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability#8-standard
chunk_level: standard
chunk_type: prose
heading: Before you begin
token_count: 380
summary: * Copy this output to a text file. You will need it later to join control plane and worker nodes to the cluster. * When `--upload-certs` is used with `kubeadm init`, the certificates of the primary...
---

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
#### Note:
You must pick a network plugin that suits your use case and deploy it before you move on to next step.
If you don't do this, you will not be able to launch your cluster properly.
* Type the following and watch the pods of the control plane components get started:
```
`kubectl get pod -n kube-system -w
`
```