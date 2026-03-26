---
doc_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability
chunk_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability#24-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 54
summary: ### Steps for the first control plane node 1. Initialize the control plane: ``` `sudo kubeadm init --control-plane-endpoint \"LOAD\_BALANCER\_DNS:LOAD\_BALANCER\_PORT\" --upload-certs ` ```
---

### Steps for the first control plane node
1. Initialize the control plane:
```
`sudo kubeadm init --control-plane-endpoint "LOAD\_BALANCER\_DNS:LOAD\_BALANCER\_PORT" --upload-certs
`
```