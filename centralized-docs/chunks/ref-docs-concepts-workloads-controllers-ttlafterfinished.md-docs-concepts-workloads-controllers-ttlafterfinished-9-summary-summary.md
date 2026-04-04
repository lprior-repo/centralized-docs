---
doc_id: ref/docs-concepts-workloads-controllers-ttlafterfinished.md/docs-concepts-workloads-controllers-ttlafterfinished
chunk_id: ref/docs-concepts-workloads-controllers-ttlafterfinished.md/docs-concepts-workloads-controllers-ttlafterfinished#9-summary
chunk_level: summary
chunk_type: prose
heading: Cleanup for finished Jobs
token_count: 89
summary: ### Time skew Because the TTL-after-finished controller uses timestamps stored in the Kubernetes jobs to determine whether the TTL has expired or not, this feature is sensitive to time skew in your...
---

### Time skew
Because the TTL-after-finished controller uses timestamps stored in the Kubernetes jobs to
determine whether the TTL has expired or not, this feature is sensitive to time
skew in your cluster, which may cause the control plane to clean up Job objects
at the wrong time.
Clocks aren't always correct, but the difference should be
very small. Please be aware of this risk when setting a non-zero TTL.