---
doc_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability
chunk_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability#36-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 115
summary: #### Note: The `kubeadm-certs` Secret and the decryption key expire after two hours. #### Caution: As stated in the command output, the certificate key gives access to cluster sensitive data, keep it...
---

#### Note:
The `kubeadm-certs` Secret and the decryption key expire after two hours.
#### Caution:
As stated in the command output, the certificate key gives access to cluster sensitive data, keep it secret!
* Apply the CNI plugin of your choice:
[Follow these instructions](/docs/setup/production-environment/tools/kubeadm/create-cluster-kubeadm/#pod-network)
to install the CNI provider. Make sure the configuration corresponds to the Pod CIDR specified in the
kubeadm configuration file (if applicable).