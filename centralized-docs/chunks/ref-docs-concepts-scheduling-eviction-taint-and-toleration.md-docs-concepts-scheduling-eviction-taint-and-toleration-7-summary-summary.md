---
doc_id: ref/docs-concepts-scheduling-eviction-taint-and-toleration.md/docs-concepts-scheduling-eviction-taint-and-toleration
chunk_id: ref/docs-concepts-scheduling-eviction-taint-and-toleration.md/docs-concepts-scheduling-eviction-taint-and-toleration#7-summary
chunk_level: summary
chunk_type: prose
heading: Concepts
token_count: 124
summary: ## Concepts You add a taint to a node using [kubectl taint](/docs/reference/generated/kubectl/kubectl-commands#taint). For example, ``` `kubectl taint nodes node1 key1=value1:NoSchedule ` ``` places...
---

## Concepts
You add a taint to a node using [kubectl taint](/docs/reference/generated/kubectl/kubectl-commands#taint).
For example,
```
`kubectl taint nodes node1 key1=value1:NoSchedule
`
```
places a taint on node `node1`. The taint has key `key1`, value `value1`, and taint effect `NoSchedule`.
This means that no pod will be able to schedule onto `node1` unless it has a matching toleration.
To remove the taint added by the command above, you can run: