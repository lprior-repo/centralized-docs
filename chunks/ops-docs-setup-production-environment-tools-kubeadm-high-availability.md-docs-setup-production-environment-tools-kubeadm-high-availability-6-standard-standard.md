---
doc_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability
chunk_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability#6-standard
chunk_level: standard
chunk_type: prose
heading: Before you begin
token_count: 278
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