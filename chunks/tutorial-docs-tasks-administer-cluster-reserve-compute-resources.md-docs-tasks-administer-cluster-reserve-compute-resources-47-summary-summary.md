---
doc_id: tutorial/docs-tasks-administer-cluster-reserve-compute-resources.md/docs-tasks-administer-cluster-reserve-compute-resources
chunk_id: tutorial/docs-tasks-administer-cluster-reserve-compute-resources.md/docs-tasks-administer-cluster-reserve-compute-resources#47-summary
chunk_level: summary
chunk_type: prose
heading: Example Scenario
token_count: 116
summary: * `evictionHard` is set to `{memory.available: \"&lt;500Mi\", nodefs.available: \"&lt;10%\"}` Under this scenario, 'Allocatable' will be 14.5 CPUs, 28.5Gi of memory and `88Gi` of local storage. Scheduler...
---

* `evictionHard` is set to `{memory.available: "&lt;500Mi", nodefs.available: "&lt;10%"}`
Under this scenario, 'Allocatable' will be 14.5 CPUs, 28.5Gi of memory and
`88Gi` of local storage.
Scheduler ensures that the total memory `requests` across all pods on this node does
not exceed 28.5Gi and storage doesn't exceed 88Gi.
Kubelet evicts pods whenever the overall memory usage across pods exceeds 28.5Gi,