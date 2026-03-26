---
doc_id: ref/docs-tasks-administer-cluster-running-cloud-controller.md/docs-tasks-administer-cluster-running-cloud-controller
chunk_id: ref/docs-tasks-administer-cluster-running-cloud-controller.md/docs-tasks-administer-cluster-running-cloud-controller#3-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 126
summary: - [this is required so CCM can bootstrap itself](#this-is-required-so-ccm-can-bootstrap-itself) - [these tolerations are to have the daemonset runnable on control plane...
---

- [this is required so CCM can bootstrap itself](#this-is-required-so-ccm-can-bootstrap-itself)
- [these tolerations are to have the daemonset runnable on control plane nodes](#these-tolerations-are-to-have-the-daemonset-runnable-on-control-plane-nodes)
- [remove them if your control plane nodes should not run pods](#remove-them-if-your-control-plane-nodes-should-not-run-pods)
- [this is to restrict CCM to only run on master nodes](#this-is-to-restrict-ccm-to-only-run-on-master-nodes)