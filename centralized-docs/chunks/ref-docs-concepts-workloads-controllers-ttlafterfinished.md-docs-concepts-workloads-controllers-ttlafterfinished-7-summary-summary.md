---
doc_id: ref/docs-concepts-workloads-controllers-ttlafterfinished.md/docs-concepts-workloads-controllers-ttlafterfinished
chunk_id: ref/docs-concepts-workloads-controllers-ttlafterfinished.md/docs-concepts-workloads-controllers-ttlafterfinished#7-summary
chunk_level: summary
chunk_type: prose
heading: Cleanup for finished Jobs
token_count: 126
summary: * Use a [mutating admission webhook](/docs/reference/access-authn-authz/admission-controllers/#mutatingadmissionwebhook) to set this field dynamically after the Job has finished, and choose different...
---

* Use a
[mutating admission webhook](/docs/reference/access-authn-authz/admission-controllers/#mutatingadmissionwebhook)
to set this field dynamically after the Job has finished, and choose
different TTL values based on job status, labels. For this case, the webhook needs
to detect changes to the `.status` of the Job and only set a TTL when the Job
is being marked as completed.
* Write your own controller to manage the cleanup TTL for Jobs that match a particular
[selector](/docs/concepts/overview/working-with-objects/labels/).## Caveats