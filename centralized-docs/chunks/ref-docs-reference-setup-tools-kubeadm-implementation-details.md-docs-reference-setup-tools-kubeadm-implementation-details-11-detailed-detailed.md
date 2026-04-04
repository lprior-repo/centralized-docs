---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#11-detailed
chunk_level: detailed
chunk_type: prose
heading: kubeadm upgrade workflow internal design
token_count: 307
summary: ## kubeadm upgrade workflow internal design `kubeadm upgrade` has sub-commands for handling the upgrade of the Kubernetes cluster created by kubeadm. You must run `kubeadm upgrade apply` on a control...
---

## kubeadm upgrade workflow internal design
`kubeadm upgrade` has sub-commands for handling the upgrade of the Kubernetes cluster created by kubeadm.
You must run `kubeadm upgrade apply` on a control plane node (you can choose which one);
this starts the upgrade process. You then run `kubeadm upgrade node` on all remaining
nodes (both worker nodes and control plane nodes).
Both `kubeadm upgrade apply` and `kubeadm upgrade node` have a `phase` subcommand which provides access
to the internal phases of the upgrade process.
See [`kubeadm upgrade phase`](/docs/reference/setup-tools/kubeadm/kubeadm-upgrade-phase/) for more details.
Additional utility upgrade commands are `kubeadm upgrade plan` and `kubeadm upgrade diff`.
All upgrade sub-commands support passing a configuration file.
### kubeadm upgrade plan
You can optionally run `kubeadm upgrade plan` before you run `kubeadm upgrade apply`.
The `plan` subcommand checks which versions are available to upgrade
to and validates whether your current cluster is upgradeable.
### kubeadm upgrade diff
This shows what differences would be applied to existing static pod manifests for control plane nodes.
A more verbose way to do the same thing is running `kubeadm upgrade apply --dry-run` or
`kubeadm upgrade node --dry-run`.