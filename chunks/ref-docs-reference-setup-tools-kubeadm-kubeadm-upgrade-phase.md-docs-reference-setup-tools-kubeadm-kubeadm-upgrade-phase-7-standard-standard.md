---
doc_id: ref/docs-reference-setup-tools-kubeadm-kubeadm-upgrade-phase.md/docs-reference-setup-tools-kubeadm-kubeadm-upgrade-phase
chunk_id: ref/docs-reference-setup-tools-kubeadm-kubeadm-upgrade-phase.md/docs-reference-setup-tools-kubeadm-kubeadm-upgrade-phase#7-standard
chunk_level: standard
chunk_type: table
heading: kubeadm upgrade node phase
token_count: 301
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
### Synopsis
Run pre-flight checks for kubeadm upgrade node.
```
`kubeadm upgrade node phase preflight [flags]
`
```
### Options
|--config string|
||
Path to a kubeadm configuration file.
|
|-h, --help|
||
help for preflight
|
|--ignore-preflight-errors strings|
||
A list of checks whose errors will be shown as warnings. Example: 'IsPrivilegedUser,Swap'. Value 'all' ignores errors from all checks.
|
### Options inherited from parent commands
|--rootfs string|
||
The path to the 'real' host root filesystem. This will cause kubeadm to chroot into the provided path.
|
### Synopsis
Upgrade the control plane instance deployed on this node, if any
```
`kubeadm upgrade node phase control-plane [flags]
`
```