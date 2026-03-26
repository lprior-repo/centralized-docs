---
doc_id: ref/docs-setup-production-environment-tools-kubeadm-kubelet-integration.md/docs-setup-production-environment-tools-kubeadm-kubelet-integration
chunk_id: ref/docs-setup-production-environment-tools-kubeadm-kubelet-integration.md/docs-setup-production-environment-tools-kubeadm-kubelet-integration#29-summary
chunk_level: summary
chunk_type: prose
heading: Configure kubelets using kubeadm
token_count: 63
summary: . The dynamic environment file is generated in exactly the same way as `kubeadm init`. Next, `kubeadm` runs the following two commands to load the new configuration into the kubelet: ``` `systemctl...
---

.
The dynamic environment file is generated in exactly the same way as `kubeadm init`.
Next, `kubeadm` runs the following two commands to load the new configuration into the kubelet:
```
`systemctl daemon-reload &amp;&amp; systemctl restart kubelet
`
```