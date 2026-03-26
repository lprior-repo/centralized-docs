---
doc_id: ref/docs-concepts-scheduling-eviction-pod-priority-preemption.md/docs-concepts-scheduling-eviction-pod-priority-preemption
chunk_id: ref/docs-concepts-scheduling-eviction-pod-priority-preemption.md/docs-concepts-scheduling-eviction-pod-priority-preemption#57-summary
chunk_level: summary
chunk_type: prose
heading: Interactions between Pod priority and quality of service
token_count: 118
summary: 1. Whether the starved resource usage exceeds requests 2. Pod Priority 3. Amount of resource usage relative to requests See [Pod selection for kubelet...
---

1. Whether the starved resource usage exceeds requests
2. Pod Priority
3. Amount of resource usage relative to requests
See [Pod selection for kubelet eviction](/docs/concepts/scheduling-eviction/node-pressure-eviction/#pod-selection-for-kubelet-eviction)
for more details.
kubelet node-pressure eviction does not evict Pods when their
usage does not exceed their requests. If a Pod with lower priority is not
exceeding its requests, it won't be evicted. Another Pod with higher priority
that exceeds its requests may be evicted.