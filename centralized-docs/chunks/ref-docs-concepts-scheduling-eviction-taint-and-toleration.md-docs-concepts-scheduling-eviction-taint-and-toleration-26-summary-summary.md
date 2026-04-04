---
doc_id: ref/docs-concepts-scheduling-eviction-taint-and-toleration.md/docs-concepts-scheduling-eviction-taint-and-toleration
chunk_id: ref/docs-concepts-scheduling-eviction-taint-and-toleration.md/docs-concepts-scheduling-eviction-taint-and-toleration#26-summary
chunk_level: summary
chunk_type: prose
heading: Numeric comparison operators
token_count: 118
summary: #### Note: When you create a Pod that uses `Gt` or `Lt` tolerations operators, the API server validates that the toleration values are valid integers. Taint values on nodes are not validated at node...
---

#### Note:
When you create a Pod that uses `Gt` or `Lt` tolerations operators, the API server validates that
the toleration values are valid integers. Taint values on nodes are not validated at node
registration time. If a node has a non-numeric taint value (for example,
`servicelevel.organization.example/agreed-service-level=high:NoSchedule`),
pods with numeric comparison operators will not match that taint and cannot schedule on that node.
For example, if nodes are tainted with a value representing a service level agreement (SLA):