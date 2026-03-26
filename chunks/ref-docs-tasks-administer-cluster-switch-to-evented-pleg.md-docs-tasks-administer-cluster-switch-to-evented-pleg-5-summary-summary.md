---
doc_id: ref/docs-tasks-administer-cluster-switch-to-evented-pleg.md/docs-tasks-administer-cluster-switch-to-evented-pleg
chunk_id: ref/docs-tasks-administer-cluster-switch-to-evented-pleg.md/docs-tasks-administer-cluster-switch-to-evented-pleg#5-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 113
summary: * The container runtime in use must support container lifecycle events. The kubelet automatically switches back to the legacy generic PLEG mechanism if the container runtime does not announce support...
---

* The container runtime in use must support container lifecycle events.
The kubelet automatically switches back to the legacy generic PLEG
mechanism if the container runtime does not announce support for
container lifecycle events, even if you have this feature gate enabled.## Why switch to Evented PLEG?
* The *Generic PLEG* incurs non-negligible overhead due to frequent polling of container statuses.
* This overhead is exacerbated by Kubelet's parallelized polling of container states, thus limiting
its scalability and causing poor performance and reliability problems.