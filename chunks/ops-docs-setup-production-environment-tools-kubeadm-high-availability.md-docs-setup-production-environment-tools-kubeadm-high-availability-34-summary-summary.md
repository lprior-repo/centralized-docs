---
doc_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability
chunk_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability#34-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 120
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