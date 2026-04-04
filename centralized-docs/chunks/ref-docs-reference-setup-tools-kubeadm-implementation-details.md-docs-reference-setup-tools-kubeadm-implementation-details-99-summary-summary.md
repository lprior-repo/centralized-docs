---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#99-summary
chunk_level: summary
chunk_type: prose
heading: kubeadm upgrade workflow internal design
token_count: 92
summary: and `kubeadm upgrade node` have a `phase` subcommand which provides access to the internal phases of the upgrade process. See [`kubeadm upgrade...
---

 and `kubeadm upgrade node` have a `phase` subcommand which provides access
to the internal phases of the upgrade process.
See [`kubeadm upgrade phase`](/docs/reference/setup-tools/kubeadm/kubeadm-upgrade-phase/) for more details.
Additional utility upgrade commands are `kubeadm upgrade plan` and `kubeadm upgrade diff`.
All upgrade sub-commands support passing a configuration file.