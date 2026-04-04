---
doc_id: ref/docs-concepts-workloads-controllers-ttlafterfinished.md/docs-concepts-workloads-controllers-ttlafterfinished
chunk_id: ref/docs-concepts-workloads-controllers-ttlafterfinished.md/docs-concepts-workloads-controllers-ttlafterfinished#6-summary
chunk_level: summary
chunk_type: prose
heading: Cleanup for finished Jobs
token_count: 103
summary: * Specify this field in the Job manifest, so that a Job can be cleaned up automatically some time after it finishes. * Manually set this field of existing, already finished Jobs, so that they become...
---

* Specify this field in the Job manifest, so that a Job can be cleaned up
automatically some time after it finishes.
* Manually set this field of existing, already finished Jobs, so that they become eligible
for cleanup.
* Use a
[mutating admission webhook](/docs/reference/access-authn-authz/admission-controllers/#mutatingadmissionwebhook)
to set this field dynamically at Job creation time. Cluster administrators can
use this to enforce a TTL policy for finished jobs.