---
doc_id: ref/docs-tasks-administer-cluster-running-cloud-controller.md/docs-tasks-administer-cluster-running-cloud-controller
chunk_id: ref/docs-tasks-administer-cluster-running-cloud-controller.md/docs-tasks-administer-cluster-running-cloud-controller#0-standard
chunk_level: standard
chunk_type: prose
heading: Table of Contents
token_count: 417
summary: ## Table of Contents  - [Cloud Controller Manager Administration](#cloud-controller-manager-administration)     - [Requirements](#requirements) - [It assumes that your masters can run pods and has...
---

## Table of Contents

- [Cloud Controller Manager Administration](#cloud-controller-manager-administration)
    - [Requirements](#requirements)
- [It assumes that your masters can run pods and has the role node-role.kubernetes.io/master](#it-assumes-that-your-masters-can-run-pods-and-has-the-role-node-rolekubernetesiomaster)
- [Note that this Daemonset will not work straight out of the box for your cloud, this is](#note-that-this-daemonset-will-not-work-straight-out-of-the-box-for-your-cloud-this-is)
- [meant to be a guideline.](#meant-to-be-a-guideline)
- [this can be replaced with any other image for out-of-tree providers](#this-can-be-replaced-with-any-other-image-for-out-of-tree-providers)
- [these flags will vary for every cloud provider](#these-flags-will-vary-for-every-cloud-provider)
- [this is required so CCM can bootstrap itself](#this-is-required-so-ccm-can-bootstrap-itself)
- [these tolerations are to have the daemonset runnable on control plane nodes](#these-tolerations-are-to-have-the-daemonset-runnable-on-control-plane-nodes)
- [remove them if your control plane nodes should not run pods](#remove-them-if-your-control-plane-nodes-should-not-run-pods)
- [this is to restrict CCM to only run on master nodes](#this-is-to-restrict-ccm-to-only-run-on-master-nodes)
- [the node selector may vary depending on your cluster setup](#the-node-selector-may-vary-depending-on-your-cluster-setup)
  - [Limitations](#limitations)
    - [Support for Volumes](#support-for-volumes)
    - [Scalability](#scalability)
    - [Chicken and Egg](#chicken-and-egg)
  - [What's next](#whats-next)
  - [Feedback](#feedback)

---