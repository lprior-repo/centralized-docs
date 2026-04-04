---
doc_id: ref/docs-concepts-scheduling-eviction-taint-and-toleration.md/docs-concepts-scheduling-eviction-taint-and-toleration
chunk_id: ref/docs-concepts-scheduling-eviction-taint-and-toleration.md/docs-concepts-scheduling-eviction-taint-and-toleration#20-summary
chunk_level: summary
chunk_type: prose
heading: Concepts
token_count: 65
summary: And a pod has two tolerations: ``` `tolerations: - key: \"key1\" operator: \"Equal\" value: \"value1\" effect: \"NoSchedule\" - key: \"key1\" operator: \"Equal\" value: \"value1\" effect: \"NoExecute\" ` ```
---

And a pod has two tolerations:
```
`tolerations:
- key: "key1"
operator: "Equal"
value: "value1"
effect: "NoSchedule"
- key: "key1"
operator: "Equal"
value: "value1"
effect: "NoExecute"
`
```