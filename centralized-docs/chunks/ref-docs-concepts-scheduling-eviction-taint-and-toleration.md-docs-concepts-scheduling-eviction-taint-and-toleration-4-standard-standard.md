---
doc_id: ref/docs-concepts-scheduling-eviction-taint-and-toleration.md/docs-concepts-scheduling-eviction-taint-and-toleration
chunk_id: ref/docs-concepts-scheduling-eviction-taint-and-toleration.md/docs-concepts-scheduling-eviction-taint-and-toleration#4-standard
chunk_level: standard
chunk_type: prose
heading: Concepts
token_count: 121
summary: #### Note: There are two special cases: If the `key` is empty, then the `operator` must be `Exists`, which matches all keys and values. Note that the `effect` still needs to be matched at the same...
---

#### Note:
There are two special cases:
If the `key` is empty, then the `operator` must be `Exists`, which matches all keys and values.
Note that the `effect` still needs to be matched at the same time.
An empty `effect` matches all effects with key `key1`.
The above example used the `effect` of `NoSchedule`. Alternatively, you can use the `effect` of `PreferNoSchedule`.
The allowed values for the `effect` field are:
`NoExecute`This affects pods that are already running on the node as follows: