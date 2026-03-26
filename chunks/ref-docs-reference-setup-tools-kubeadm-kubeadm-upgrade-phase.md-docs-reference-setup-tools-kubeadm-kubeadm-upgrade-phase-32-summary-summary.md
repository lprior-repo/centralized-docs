---
doc_id: ref/docs-reference-setup-tools-kubeadm-kubeadm-upgrade-phase.md/docs-reference-setup-tools-kubeadm-kubeadm-upgrade-phase
chunk_id: ref/docs-reference-setup-tools-kubeadm-kubeadm-upgrade-phase.md/docs-reference-setup-tools-kubeadm-kubeadm-upgrade-phase#32-summary
chunk_level: summary
chunk_type: table
heading: kubeadm upgrade node phase
token_count: 107
summary: ### Synopsis Run pre-flight checks for kubeadm upgrade node. ``` `kubeadm upgrade node phase preflight [flags] ` ``` ### Options |--config string| || Path to a kubeadm configuration file. | |-h,...
---

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