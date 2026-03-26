---
doc_id: ref/docs-reference-setup-tools-kubeadm-kubeadm-upgrade-phase.md/docs-reference-setup-tools-kubeadm-kubeadm-upgrade-phase
chunk_id: ref/docs-reference-setup-tools-kubeadm-kubeadm-upgrade-phase.md/docs-reference-setup-tools-kubeadm-kubeadm-upgrade-phase#10-standard
chunk_level: standard
chunk_type: table
heading: What's next
token_count: 414
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
### Options
|--config string|
||
Path to a kubeadm configuration file.
|
|--dry-run|
||
Do not change any state, just output the actions that would be performed.
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
## What's next
* [kubeadm init](/docs/reference/setup-tools/kubeadm/kubeadm-init/) to bootstrap a Kubernetes control-plane node
* [kubeadm join](/docs/reference/setup-tools/kubeadm/kubeadm-join/) to connect a node to the cluster
* [kubeadm reset](/docs/reference/setup-tools/kubeadm/kubeadm-reset/) to revert any changes made to this host by `kubeadm init` or `kubeadm join`
* [kubeadm upgrade](/docs/reference/setup-tools/kubeadm/kubeadm-upgrade/) to upgrade a kubeadm node
* [kubeadm alpha](/docs/reference/setup-tools/kubeadm/kubeadm-alpha/) to try experimental functionality