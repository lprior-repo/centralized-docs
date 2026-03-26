---
doc_id: ref/docs-reference-setup-tools-kubeadm-kubeadm-upgrade-phase.md/docs-reference-setup-tools-kubeadm-kubeadm-upgrade-phase
chunk_id: ref/docs-reference-setup-tools-kubeadm-kubeadm-upgrade-phase.md/docs-reference-setup-tools-kubeadm-kubeadm-upgrade-phase#12-summary
chunk_level: summary
chunk_type: table
heading: kubeadm upgrade apply phase
token_count: 96
summary: | |-f, --force| || Force upgrading although some requirements might not be met. This also implies non-interactive mode. | |-h, --help| || help for preflight | |--ignore-preflight-errors strings| || A...
---

|
|-f, --force|
||
Force upgrading although some requirements might not be met. This also implies non-interactive mode.
|
|-h, --help|
||
help for preflight
|
|--ignore-preflight-errors strings|
||
A list of checks whose errors will be shown as warnings. Example: 'IsPrivilegedUser,Swap'. Value 'all' ignores errors from all checks.
|
|--kubeconfig stringDefault: "/etc/kubernetes/admin.conf"|
||