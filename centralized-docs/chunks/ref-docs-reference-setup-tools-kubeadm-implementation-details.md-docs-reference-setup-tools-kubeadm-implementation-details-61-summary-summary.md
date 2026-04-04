---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#61-summary
chunk_level: summary
chunk_type: prose
heading: kubeadm init workflow internal design
token_count: 121
summary: * `--use-service-account-credentials` to `true` * Flags for using certificates generated in previous steps: * `--root-ca-file` to `ca.crt` * `--cluster-signing-cert-file` to `ca.crt`, if External CA...
---

* `--use-service-account-credentials` to `true`
* Flags for using certificates generated in previous steps:
* `--root-ca-file` to `ca.crt`
* `--cluster-signing-cert-file` to `ca.crt`, if External CA mode is disabled, otherwise to `""`
* `--cluster-signing-key-file` to `ca.key`, if External CA mode is disabled, otherwise to `""`
* `--service-account-private-key-file` to `sa.key`#### Scheduler
The static Pod manifest for the scheduler is not affected by parameters provided by the user.