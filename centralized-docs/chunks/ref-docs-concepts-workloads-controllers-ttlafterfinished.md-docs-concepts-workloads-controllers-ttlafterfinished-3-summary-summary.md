---
doc_id: ref/docs-concepts-workloads-controllers-ttlafterfinished.md/docs-concepts-workloads-controllers-ttlafterfinished
chunk_id: ref/docs-concepts-workloads-controllers-ttlafterfinished.md/docs-concepts-workloads-controllers-ttlafterfinished#3-summary
chunk_level: summary
chunk_type: prose
heading: Cleanup for finished Jobs
token_count: 128
summary: The TTL-after-finished controller is only supported for Jobs. You can use this mechanism to clean up finished Jobs (either `Complete` or `Failed`) automatically by specifying the...
---

The TTL-after-finished controller is only supported for Jobs. You can use this mechanism to clean
up finished Jobs (either `Complete` or `Failed`) automatically by specifying the
`.spec.ttlSecondsAfterFinished` field of a Job, as in this
[example](/docs/concepts/workloads/controllers/job/#clean-up-finished-jobs-automatically).
The TTL-after-finished controller assumes that a Job is eligible to be cleaned up
TTL seconds after the Job has finished. The timer starts once the
status condition of the Job changes to show that the Job is either `Complete` or `Failed`