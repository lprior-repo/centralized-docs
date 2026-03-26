---
doc_id: ref/docs-reference-setup-tools-kubeadm-kubeadm-upgrade-phase.md/docs-reference-setup-tools-kubeadm-kubeadm-upgrade-phase
chunk_id: ref/docs-reference-setup-tools-kubeadm-kubeadm-upgrade-phase.md/docs-reference-setup-tools-kubeadm-kubeadm-upgrade-phase#3-detailed
chunk_level: detailed
chunk_type: table
heading: kubeadm upgrade node phase
token_count: 1019
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
### Options
|--certificate-renewalDefault: true|
||
Perform the renewal of certificates used by component changed during upgrades.
|
|--config string|
||
Path to a kubeadm configuration file.
|
|--dry-run|
||
Do not change any state, just output the actions that would be performed.
|
|--etcd-upgradeDefault: true|
||
Perform the upgrade of etcd.
|
|-h, --help|
||
help for control-plane
|
|--kubeconfig stringDefault: "/etc/kubernetes/admin.conf"|
||
The kubeconfig file to use when talking to the cluster. If the flag is not set, a set of standard locations can be searched for an existing kubeconfig file.
|
|--patches string|
||
Path to a directory that contains files named "target[suffix][+patchtype].extension". For example, "kube-apiserver0+merge.yaml" or just "etcd.json". "target" can be one of "kube-apiserver", "kube-controller-manager", "kube-scheduler", "etcd", "kubeletconfiguration", "corednsdeployment". "patchtype" can be one of "strategic", "merge" or "json" and they match the patch formats supported by kubectl. The default "patchtype" is "strategic". "extension" must be either "json" or "yaml". "suffix" is an optional string that can be used to determine which patches are applied first alpha-numerically.
|
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
help for kubelet-config
|
|--kubeconfig stringDefault: "/etc/kubernetes/admin.conf"|
||
The kubeconfig file to use when talking to the cluster. If the flag is not set, a set of standard locations can be searched for an existing kubeconfig file.
|
|--patches string|
||
Path to a directory that contains files named "target[suffix][+patchtype].extension". For example, "kube-apiserver0+merge.yaml" or just "etcd.json". "target" can be one of "kube-apiserver", "kube-controller-manager", "kube-scheduler", "etcd", "kubeletconfiguration", "corednsdeployment". "patchtype" can be one of "strategic", "merge" or "json" and they match the patch formats supported by kubectl. The default "patchtype" is "strategic". "extension" must be either "json" or "yaml". "suffix" is an optional string that can be used to determine which patches are applied first alpha-numerically.
|
### Options inherited from parent commands
|--rootfs string|
||
The path to the 'real' host root filesystem. This will cause kubeadm to chroot into the provided path.
|
### Synopsis
Upgrade the default kubeadm addons
```
`kubeadm upgrade node phase addon [flags]
`
```