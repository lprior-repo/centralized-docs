---
doc_id: ref/docs-setup-production-environment-tools-kubeadm-kubelet-integration.md/docs-setup-production-environment-tools-kubeadm-kubelet-integration
chunk_id: ref/docs-setup-production-environment-tools-kubeadm-kubelet-integration.md/docs-setup-production-environment-tools-kubeadm-kubelet-integration#25-summary
chunk_level: summary
chunk_type: prose
heading: Configure kubelets using kubeadm
token_count: 102
summary: . To address the second pattern of [providing instance-specific configuration details](#providing-instance-specific-configuration-details), kubeadm writes an environment file to...
---

.
To address the second pattern of
[providing instance-specific configuration details](#providing-instance-specific-configuration-details),
kubeadm writes an environment file to `/var/lib/kubelet/kubeadm-flags.env`, which contains a list of
flags to pass to the kubelet when it starts. The flags are presented in the file like this:
```
`KUBELET\_KUBEADM\_ARGS="--flag1=value1 --flag2=value2 ..."
`
```