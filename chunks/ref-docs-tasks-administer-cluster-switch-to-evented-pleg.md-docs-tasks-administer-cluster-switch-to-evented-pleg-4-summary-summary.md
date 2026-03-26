---
doc_id: ref/docs-tasks-administer-cluster-switch-to-evented-pleg.md/docs-tasks-administer-cluster-switch-to-evented-pleg
chunk_id: ref/docs-tasks-administer-cluster-switch-to-evented-pleg.md/docs-tasks-administer-cluster-switch-to-evented-pleg#4-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 95
summary: * You need to run a version of Kubernetes that provides this feature. Kubernetes v1.27 includes beta support for event-based container status updates. The feature is beta but is *disabled* by default...
---

* You need to run a version of Kubernetes that provides this feature.
Kubernetes v1.27 includes beta support for event-based container
status updates. The feature is beta but is *disabled* by default
because it requires support from the container runtime.
* Your Kubernetes server must be at or later than version 1.26.
To check the version, enter `kubectl version`.
If you are running a different version of Kubernetes, check the documentation for that release.