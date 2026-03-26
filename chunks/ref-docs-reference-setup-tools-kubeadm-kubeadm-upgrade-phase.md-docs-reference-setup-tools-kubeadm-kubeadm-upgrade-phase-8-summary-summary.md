---
doc_id: ref/docs-reference-setup-tools-kubeadm-kubeadm-upgrade-phase.md/docs-reference-setup-tools-kubeadm-kubeadm-upgrade-phase
chunk_id: ref/docs-reference-setup-tools-kubeadm-kubeadm-upgrade-phase.md/docs-reference-setup-tools-kubeadm-kubeadm-upgrade-phase#8-summary
chunk_level: summary
chunk_type: table
heading: kubeadm upgrade apply phase
token_count: 113
summary: ## kubeadm upgrade apply phase Using the phases of `kubeadm upgrade apply`, you can choose to execute the separate steps of the initial upgrade of a control plane node. ### Synopsis Use this command...
---

## kubeadm upgrade apply phase
Using the phases of `kubeadm upgrade apply`, you can choose to execute the separate steps of the initial upgrade
of a control plane node.
### Synopsis
Use this command to invoke single phase of the "apply" workflow
```
`kubeadm upgrade apply phase [flags]
`
```
### Options inherited from parent commands
|--rootfs string|
||
The path to the 'real' host root filesystem. This will cause kubeadm to chroot into the provided path.
|