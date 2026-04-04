---
doc_id: ref/docs-concepts-scheduling-eviction-taint-and-toleration.md/docs-concepts-scheduling-eviction-taint-and-toleration
chunk_id: ref/docs-concepts-scheduling-eviction-taint-and-toleration.md/docs-concepts-scheduling-eviction-taint-and-toleration#29-summary
chunk_level: summary
chunk_type: prose
heading: Numeric comparison operators
token_count: 110
summary: This toleration matches the taint on `node1` because `950 &gt; 900` (the taint value is greater than the toleration value for the `Gt` operator). Similarly, you can use the `Lt` operator to match...
---

This toleration matches the taint on `node1` because `950 &gt; 900` (the taint value
is greater than the toleration value for the `Gt` operator).
Similarly, you can use the `Lt` operator to match taints where the taint value is
less than the toleration value:
```
`tolerations:
- key: "servicelevel.organization.example/agreed-service-level"
operator: "Lt"
value: "1000"
effect: "NoSchedule"
`
```