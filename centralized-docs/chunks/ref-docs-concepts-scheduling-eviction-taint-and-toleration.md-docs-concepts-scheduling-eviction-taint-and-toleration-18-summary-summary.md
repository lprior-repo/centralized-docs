---
doc_id: ref/docs-concepts-scheduling-eviction-taint-and-toleration.md/docs-concepts-scheduling-eviction-taint-and-toleration
chunk_id: ref/docs-concepts-scheduling-eviction-taint-and-toleration.md/docs-concepts-scheduling-eviction-taint-and-toleration#18-summary
chunk_level: summary
chunk_type: prose
heading: Concepts
token_count: 80
summary: * if there is at least one un-ignored taint with effect `NoSchedule` then Kubernetes will not schedule the pod onto that node * if there is no un-ignored taint with effect `NoSchedule` but there is...
---

* if there is at least one un-ignored taint with effect `NoSchedule` then Kubernetes will not schedule
the pod onto that node
* if there is no un-ignored taint with effect `NoSchedule` but there is at least one un-ignored taint with
effect `PreferNoSchedule` then Kubernetes will *try* to not schedule the pod onto the node