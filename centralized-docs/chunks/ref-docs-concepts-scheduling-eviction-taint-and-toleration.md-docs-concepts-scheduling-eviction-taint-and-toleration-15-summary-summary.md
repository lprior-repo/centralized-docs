---
doc_id: ref/docs-concepts-scheduling-eviction-taint-and-toleration.md/docs-concepts-scheduling-eviction-taint-and-toleration
chunk_id: ref/docs-concepts-scheduling-eviction-taint-and-toleration.md/docs-concepts-scheduling-eviction-taint-and-toleration#15-summary
chunk_level: summary
chunk_type: prose
heading: Concepts
token_count: 37
summary: * Pods that do not tolerate the taint are evicted immediately * Pods that tolerate the taint without specifying `tolerationSeconds` in their toleration specification remain bound forever
---

* Pods that do not tolerate the taint are evicted immediately
* Pods that tolerate the taint without specifying `tolerationSeconds` in
their toleration specification remain bound forever