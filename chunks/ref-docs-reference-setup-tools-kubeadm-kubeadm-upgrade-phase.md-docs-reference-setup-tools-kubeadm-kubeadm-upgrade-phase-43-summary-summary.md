---
doc_id: ref/docs-reference-setup-tools-kubeadm-kubeadm-upgrade-phase.md/docs-reference-setup-tools-kubeadm-kubeadm-upgrade-phase
chunk_id: ref/docs-reference-setup-tools-kubeadm-kubeadm-upgrade-phase.md/docs-reference-setup-tools-kubeadm-kubeadm-upgrade-phase#43-summary
chunk_level: summary
chunk_type: table
heading: kubeadm upgrade node phase
token_count: 95
summary: ### Synopsis Upgrade the default kubeadm addons ``` `kubeadm upgrade node phase addon [flags] ` ``` ### Options inherited from parent commands |--rootfs string| || The path to the 'real' host root...
---

### Synopsis
Upgrade the default kubeadm addons
```
`kubeadm upgrade node phase addon [flags]
`
```
### Options inherited from parent commands
|--rootfs string|
||
The path to the 'real' host root filesystem. This will cause kubeadm to chroot into the provided path.
|
### Synopsis
Run post upgrade tasks
```
`kubeadm upgrade node phase post-upgrade [flags]
`
```