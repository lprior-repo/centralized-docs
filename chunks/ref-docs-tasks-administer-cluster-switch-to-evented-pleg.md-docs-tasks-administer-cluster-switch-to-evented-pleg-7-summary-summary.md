---
doc_id: ref/docs-tasks-administer-cluster-switch-to-evented-pleg.md/docs-tasks-administer-cluster-switch-to-evented-pleg
chunk_id: ref/docs-tasks-administer-cluster-switch-to-evented-pleg.md/docs-tasks-administer-cluster-switch-to-evented-pleg#7-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 111
summary: 1. Start the Kubelet with the [feature gate](/docs/reference/command-line-tools-reference/feature-gates/) `EventedPLEG` enabled. You can manage the kubelet feature gates editing the kubelet [config...
---

1. Start the Kubelet with the [feature gate](/docs/reference/command-line-tools-reference/feature-gates/)
`EventedPLEG` enabled. You can manage the kubelet feature gates editing the kubelet
[config file](/docs/tasks/administer-cluster/kubelet-config-file/) and restarting the kubelet service.
You need to do this on each node where you are using this feature.
2. Make sure the node is [drained](/docs/tasks/administer-cluster/safely-drain-node/) before proceeding.