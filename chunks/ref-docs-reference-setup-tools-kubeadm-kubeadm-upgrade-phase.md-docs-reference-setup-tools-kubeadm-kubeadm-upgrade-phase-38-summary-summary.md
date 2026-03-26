---
doc_id: ref/docs-reference-setup-tools-kubeadm-kubeadm-upgrade-phase.md/docs-reference-setup-tools-kubeadm-kubeadm-upgrade-phase
chunk_id: ref/docs-reference-setup-tools-kubeadm-kubeadm-upgrade-phase.md/docs-reference-setup-tools-kubeadm-kubeadm-upgrade-phase#38-summary
chunk_level: summary
chunk_type: table
heading: kubeadm upgrade node phase
token_count: 84
summary: ### Options inherited from parent commands |--rootfs string| || The path to the 'real' host root filesystem. This will cause kubeadm to chroot into the provided path. | ### Synopsis Upgrade the...
---

### Options inherited from parent commands
|--rootfs string|
||
The path to the 'real' host root filesystem. This will cause kubeadm to chroot into the provided path.
|
### Synopsis
Upgrade the kubelet configuration for this node by downloading it from the kubelet-config ConfigMap stored in the cluster
```
`kubeadm upgrade node phase kubelet-config [flags]
`
```