---
doc_id: ref/docs-concepts-scheduling-eviction-pod-scheduling-readiness.md/docs-concepts-scheduling-eviction-pod-scheduling-readiness
chunk_id: ref/docs-concepts-scheduling-eviction-pod-scheduling-readiness.md/docs-concepts-scheduling-eviction-pod-scheduling-readiness#21-summary
chunk_level: summary
chunk_type: prose
heading: What's next
token_count: 85
summary: 4. For `.preferredDuringSchedulingIgnoredDuringExecution`, all updates are allowed. This is because preferred terms are not authoritative, and so policy controllers don't validate those terms. ##...
---

4. For `.preferredDuringSchedulingIgnoredDuringExecution`, all updates are allowed.
This is because preferred terms are not authoritative, and so policy controllers
don't validate those terms.
## What's next
* Read the [PodSchedulingReadiness KEP](https://github.com/kubernetes/enhancements/blob/master/keps/sig-scheduling/3521-pod-scheduling-readiness) for more details