---
doc_id: ref/docs-concepts-scheduling-eviction-taint-and-toleration.md/docs-concepts-scheduling-eviction-taint-and-toleration
chunk_id: ref/docs-concepts-scheduling-eviction-taint-and-toleration.md/docs-concepts-scheduling-eviction-taint-and-toleration#8-summary
chunk_level: summary
chunk_type: prose
heading: Concepts
token_count: 110
summary: ``` `kubectl taint nodes node1 key1=value1:NoSchedule- ` ``` You specify a toleration for a pod in the PodSpec. Both of the following tolerations \"match\" the taint created by the `kubectl taint` line...
---

```
`kubectl taint nodes node1 key1=value1:NoSchedule-
`
```
You specify a toleration for a pod in the PodSpec. Both of the following tolerations "match" the
taint created by the `kubectl taint` line above, and thus a pod with either toleration would be able
to schedule onto `node1`:
```
`tolerations:
- key: "key1"
operator: "Equal"
value: "value1"
effect: "NoSchedule"
`
```