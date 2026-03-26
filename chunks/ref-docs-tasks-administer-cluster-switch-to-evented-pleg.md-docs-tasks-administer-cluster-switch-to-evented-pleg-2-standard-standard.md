---
doc_id: ref/docs-tasks-administer-cluster-switch-to-evented-pleg.md/docs-tasks-administer-cluster-switch-to-evented-pleg
chunk_id: ref/docs-tasks-administer-cluster-switch-to-evented-pleg.md/docs-tasks-administer-cluster-switch-to-evented-pleg#2-standard
chunk_level: standard
chunk_type: prose
heading: Before you begin
token_count: 497
summary: ## Before you begin * You need to run a version of Kubernetes that provides this feature. Kubernetes v1.27 includes beta support for event-based container status updates. The feature is beta but is...
---

## Before you begin
* You need to run a version of Kubernetes that provides this feature.
Kubernetes v1.27 includes beta support for event-based container
status updates. The feature is beta but is *disabled* by default
because it requires support from the container runtime.
* Your Kubernetes server must be at or later than version 1.26.
To check the version, enter `kubectl version`.
If you are running a different version of Kubernetes, check the documentation for that release.
* The container runtime in use must support container lifecycle events.
The kubelet automatically switches back to the legacy generic PLEG
mechanism if the container runtime does not announce support for
container lifecycle events, even if you have this feature gate enabled.## Why switch to Evented PLEG?
* The *Generic PLEG* incurs non-negligible overhead due to frequent polling of container statuses.
* This overhead is exacerbated by Kubelet's parallelized polling of container states, thus limiting
its scalability and causing poor performance and reliability problems.
* The goal of *Evented PLEG* is to reduce unnecessary work during inactivity
by replacing periodic polling.## Switching to Evented PLEG
1. Start the Kubelet with the [feature gate](/docs/reference/command-line-tools-reference/feature-gates/)
`EventedPLEG` enabled. You can manage the kubelet feature gates editing the kubelet
[config file](/docs/tasks/administer-cluster/kubelet-config-file/) and restarting the kubelet service.
You need to do this on each node where you are using this feature.
2. Make sure the node is [drained](/docs/tasks/administer-cluster/safely-drain-node/) before proceeding.
3. Start the container runtime with the container event generation enabled.
Version 1.7+
Version 1.26+
Check if the CRI-O is already configured to emit CRI events by verifying the configuration,
```
`crio config | grep enable\_pod\_events
`
```
If it is enabled, the output should be similar to the following:
```
`enable\_pod\_events = true
`
```
To enable it, start the CRI-O daemon with the flag `--enable-pod-events=true` or
use a dropin config with the following lines:
```
`[crio.runtime]
enable\_pod\_events: true
`
```