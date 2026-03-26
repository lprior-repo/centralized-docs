---
doc_id: ref/docs-reference-setup-tools-kubeadm-kubeadm-upgrade-phase.md/docs-reference-setup-tools-kubeadm-kubeadm-upgrade-phase
chunk_id: ref/docs-reference-setup-tools-kubeadm-kubeadm-upgrade-phase.md/docs-reference-setup-tools-kubeadm-kubeadm-upgrade-phase#6-standard
chunk_level: standard
chunk_type: table
heading: kubeadm upgrade apply phase
token_count: 451
summary: ### Options inherited from parent commands |--rootfs string| || The path to the 'real' host root filesystem. This will cause kubeadm to chroot into the provided path. | ### Synopsis Configures...
---

### Options inherited from parent commands
|--rootfs string|
||
The path to the 'real' host root filesystem. This will cause kubeadm to chroot into the provided path.
|
### Synopsis
Configures bootstrap token and cluster-info RBAC rules
```
`kubeadm upgrade apply phase bootstrap-token [flags]
`
```
### Options
|--config string|
||
Path to a kubeadm configuration file.
|
|--dry-run|
||
Do not change any state, just output what actions would be performed.
|
|-h, --help|
||
help for bootstrap-token
|
|--kubeconfig stringDefault: "/etc/kubernetes/admin.conf"|
||
The kubeconfig file to use when talking to the cluster. If the flag is not set, a set of standard locations can be searched for an existing kubeconfig file.
|
### Options inherited from parent commands
|--rootfs string|
||
The path to the 'real' host root filesystem. This will cause kubeadm to chroot into the provided path.
|
### Synopsis
Upgrade the default kubeadm addons
```
`kubeadm upgrade apply phase addon [flags]
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
`kubeadm upgrade apply phase post-upgrade [flags]
`
```
### Options
|--config string|
||
Path to a kubeadm configuration file.
|
|--dry-run|
||
Do not change any state, just output what actions would be performed.
|
|-h, --help|
||
help for post-upgrade
|
|--kubeconfig stringDefault: "/etc/kubernetes/admin.conf"|
||
The kubeconfig file to use when talking to the cluster. If the flag is not set, a set of standard locations can be searched for an existing kubeconfig file.
|
### Options inherited from parent commands
|--rootfs string|
||
The path to the 'real' host root filesystem. This will cause kubeadm to chroot into the provided path.
|