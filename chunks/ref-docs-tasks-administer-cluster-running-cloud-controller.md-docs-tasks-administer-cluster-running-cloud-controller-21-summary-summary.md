---
doc_id: ref/docs-tasks-administer-cluster-running-cloud-controller.md/docs-tasks-administer-cluster-running-cloud-controller
chunk_id: ref/docs-tasks-administer-cluster-running-cloud-controller.md/docs-tasks-administer-cluster-running-cloud-controller#21-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 107
summary: # these tolerations are to have the daemonset runnable on control plane nodes # remove them if your control plane nodes should not run pods - key: node-role.kubernetes.io/control-plane operator:...
---

# these tolerations are to have the daemonset runnable on control plane nodes
# remove them if your control plane nodes should not run pods
- key: node-role.kubernetes.io/control-plane
operator: Exists
effect: NoSchedule
- key: node-role.kubernetes.io/master
operator: Exists
effect: NoSchedule
# this is to restrict CCM to only run on master nodes
# the node selector may vary depending on your cluster setup
nodeSelector:
node-role.kubernetes.io/master: ""
`
```