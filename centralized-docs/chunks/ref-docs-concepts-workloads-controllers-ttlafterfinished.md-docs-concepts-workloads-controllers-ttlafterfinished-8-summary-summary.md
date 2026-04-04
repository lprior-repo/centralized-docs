---
doc_id: ref/docs-concepts-workloads-controllers-ttlafterfinished.md/docs-concepts-workloads-controllers-ttlafterfinished
chunk_id: ref/docs-concepts-workloads-controllers-ttlafterfinished.md/docs-concepts-workloads-controllers-ttlafterfinished#8-summary
chunk_level: summary
chunk_type: prose
heading: Cleanup for finished Jobs
token_count: 83
summary: ### Updating TTL for finished Jobs You can modify the TTL period, e.g. `.spec.ttlSecondsAfterFinished` field of Jobs, after the job is created or has finished. If you extend the TTL period after the...
---

### Updating TTL for finished Jobs
You can modify the TTL period, e.g. `.spec.ttlSecondsAfterFinished` field of Jobs,
after the job is created or has finished. If you extend the TTL period after the
existing `ttlSecondsAfterFinished` period has expired, Kubernetes doesn't guarantee
to retain that Job, even if an update to extend the TTL returns a successful API
response.