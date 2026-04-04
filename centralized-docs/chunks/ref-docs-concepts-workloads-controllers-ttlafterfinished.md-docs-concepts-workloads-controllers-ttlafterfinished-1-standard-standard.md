---
doc_id: ref/docs-concepts-workloads-controllers-ttlafterfinished.md/docs-concepts-workloads-controllers-ttlafterfinished
chunk_id: ref/docs-concepts-workloads-controllers-ttlafterfinished.md/docs-concepts-workloads-controllers-ttlafterfinished#1-standard
chunk_level: standard
chunk_type: prose
heading: Table of Contents
token_count: 114
summary: # Automatic Cleanup for Finished Jobs A time-to-live mechanism to clean up old Jobs that have finished execution. FEATURE STATE: `Kubernetes v1.23 [stable]` When your Job has finished, it's useful to...
---

# Automatic Cleanup for Finished Jobs
A time-to-live mechanism to clean up old Jobs that have finished execution.
FEATURE STATE:
`Kubernetes v1.23 [stable]`
When your Job has finished, it's useful to keep that Job in the API (and not immediately delete the Job)
so that you can tell whether the Job succeeded or failed.
Kubernetes' TTL-after-finished [controller](/docs/concepts/architecture/controller/) provides a
TTL (time to live) mechanism to limit the lifetime of Job objects that
have finished execution.