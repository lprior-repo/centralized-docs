---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#100-summary
chunk_level: summary
chunk_type: prose
heading: kubeadm upgrade workflow internal design
token_count: 116
summary: ### kubeadm upgrade plan You can optionally run `kubeadm upgrade plan` before you run `kubeadm upgrade apply`. The `plan` subcommand checks which versions are available to upgrade to and validates...
---

### kubeadm upgrade plan
You can optionally run `kubeadm upgrade plan` before you run `kubeadm upgrade apply`.
The `plan` subcommand checks which versions are available to upgrade
to and validates whether your current cluster is upgradeable.
### kubeadm upgrade diff
This shows what differences would be applied to existing static pod manifests for control plane nodes.
A more verbose way to do the same thing is running `kubeadm upgrade apply --dry-run` or
`kubeadm upgrade node --dry-run`.