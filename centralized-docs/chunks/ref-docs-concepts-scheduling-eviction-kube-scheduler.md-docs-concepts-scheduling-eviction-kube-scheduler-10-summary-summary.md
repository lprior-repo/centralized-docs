---
doc_id: ref/docs-concepts-scheduling-eviction-kube-scheduler.md/docs-concepts-scheduling-eviction-kube-scheduler
chunk_id: ref/docs-concepts-scheduling-eviction-kube-scheduler.md/docs-concepts-scheduling-eviction-kube-scheduler#10-summary
chunk_level: summary
chunk_type: prose
heading: kube-scheduler
token_count: 99
summary: *scoring* step, the scheduler ranks the remaining nodes to choose the most suitable Pod placement. The scheduler assigns a score to each Node that survived filtering, basing this score on the active...
---

*scoring* step, the scheduler ranks the remaining nodes to choose
the most suitable Pod placement. The scheduler assigns a score to each Node
that survived filtering, basing this score on the active scoring rules.
Finally, kube-scheduler assigns the Pod to the Node with the highest ranking.
If there is more than one node with equal scores, kube-scheduler selects
one of these at random.
There are two supported ways to configure the filtering and scoring behavior
of the scheduler: