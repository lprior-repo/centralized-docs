---
doc_id: ref/docs-reference-setup-tools-kubeadm-kubeadm-upgrade-phase.md/docs-reference-setup-tools-kubeadm-kubeadm-upgrade-phase
chunk_id: ref/docs-reference-setup-tools-kubeadm-kubeadm-upgrade-phase.md/docs-reference-setup-tools-kubeadm-kubeadm-upgrade-phase#31-summary
chunk_level: summary
chunk_type: table
heading: kubeadm upgrade node phase
token_count: 120
summary: ## kubeadm upgrade node phase Using the phases of `kubeadm upgrade node` you can choose to execute the separate steps of the upgrade of secondary control-plane or worker nodes. ### Synopsis Use this...
---

## kubeadm upgrade node phase
Using the phases of `kubeadm upgrade node` you can choose to execute the separate steps of the upgrade of
secondary control-plane or worker nodes.
### Synopsis
Use this command to invoke single phase of the "node" workflow
```
`kubeadm upgrade node phase [flags]
`
```
### Options inherited from parent commands
|--rootfs string|
||
The path to the 'real' host root filesystem. This will cause kubeadm to chroot into the provided path.
|
Run upgrade node pre-flight checks