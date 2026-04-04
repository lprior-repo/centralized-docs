---
doc_id: ref/docs-concepts-workloads-controllers-ttlafterfinished.md/docs-concepts-workloads-controllers-ttlafterfinished
chunk_id: ref/docs-concepts-workloads-controllers-ttlafterfinished.md/docs-concepts-workloads-controllers-ttlafterfinished#2-standard
chunk_level: standard
chunk_type: prose
heading: Cleanup for finished Jobs
token_count: 502
summary: ## Cleanup for finished Jobs The TTL-after-finished controller is only supported for Jobs. You can use this mechanism to clean up finished Jobs (either `Complete` or `Failed`) automatically by...
---

## Cleanup for finished Jobs
The TTL-after-finished controller is only supported for Jobs. You can use this mechanism to clean
up finished Jobs (either `Complete` or `Failed`) automatically by specifying the
`.spec.ttlSecondsAfterFinished` field of a Job, as in this
[example](/docs/concepts/workloads/controllers/job/#clean-up-finished-jobs-automatically).
The TTL-after-finished controller assumes that a Job is eligible to be cleaned up
TTL seconds after the Job has finished. The timer starts once the
status condition of the Job changes to show that the Job is either `Complete` or `Failed`; once the TTL has
expired, that Job becomes eligible for
[cascading](/docs/concepts/architecture/garbage-collection/#cascading-deletion) removal. When the
TTL-after-finished controller cleans up a job, it will delete it cascadingly, that is to say it will delete
its dependent objects together with it.
Kubernetes honors object lifecycle guarantees on the Job, such as waiting for
[finalizers](/docs/concepts/overview/working-with-objects/finalizers/).
You can set the TTL seconds at any time. Here are some examples for setting the
`.spec.ttlSecondsAfterFinished` field of a Job:
* Specify this field in the Job manifest, so that a Job can be cleaned up
automatically some time after it finishes.
* Manually set this field of existing, already finished Jobs, so that they become eligible
for cleanup.
* Use a
[mutating admission webhook](/docs/reference/access-authn-authz/admission-controllers/#mutatingadmissionwebhook)
to set this field dynamically at Job creation time. Cluster administrators can
use this to enforce a TTL policy for finished jobs.
* Use a
[mutating admission webhook](/docs/reference/access-authn-authz/admission-controllers/#mutatingadmissionwebhook)
to set this field dynamically after the Job has finished, and choose
different TTL values based on job status, labels. For this case, the webhook needs
to detect changes to the `.status` of the Job and only set a TTL when the Job
is being marked as completed.
* Write your own controller to manage the cleanup TTL for Jobs that match a particular
[selector](/docs/concepts/overview/working-with-objects/labels/).## Caveats