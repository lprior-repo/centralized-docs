---
doc_id: ref/docs-tasks-debug-debug-cluster-audit.md/docs-tasks-debug-debug-cluster-audit
chunk_id: ref/docs-tasks-debug-debug-cluster-audit.md/docs-tasks-debug-debug-cluster-audit#12-standard
chunk_level: standard
chunk_type: prose
heading: What's next
token_count: 489
summary: ## Parameter tuning Parameters should be set to accommodate the load on the API server. For example, if kube-apiserver receives 100 requests each second, and each request is audited only on...
---

## Parameter tuning
Parameters should be set to accommodate the load on the API server.
For example, if kube-apiserver receives 100 requests each second, and each request is audited only
on `ResponseStarted` and `ResponseComplete` stages, you should account for ≅200 audit
events being generated each second. Assuming that there are up to 100 events in a batch,
you should set throttling level at least 2 queries per second. Assuming that the backend can take up to
5 seconds to write events, you should set the buffer size to hold up to 5 seconds of events;
that is: 10 batches, or 1000 events.
In most cases however, the default parameters should be sufficient and you don't have to worry about
setting them manually. You can look at the following Prometheus metrics exposed by kube-apiserver
and in the logs to monitor the state of the auditing subsystem.
* `apiserver\_audit\_event\_total` metric contains the total number of audit events exported.
* `apiserver\_audit\_error\_total` metric contains the total number of events dropped due to an error
during exporting.### Log entry truncation
Both log and webhook backends support limiting the size of events that are logged.
As an example, the following is the list of flags available for the log backend:
* `audit-log-truncate-enabled` whether event and batch truncating is enabled.
* `audit-log-truncate-max-batch-size` maximum size in bytes of the batch sent to the underlying backend.
* `audit-log-truncate-max-event-size` maximum size in bytes of the audit event sent to the underlying backend.
By default truncate is disabled in both `webhook` and `log`, a cluster administrator should set
`audit-log-truncate-enabled` or `audit-webhook-truncate-enabled` to enable the feature.
## What's next
* Learn about [Mutating webhook auditing annotations](/docs/reference/access-authn-authz/extensible-admission-controllers/#mutating-webhook-auditing-annotations).
* Learn more about [`Event`](/docs/reference/config-api/apiserver-audit.v1/#audit-k8s-io-v1-Event)
and the [`Policy`](/docs/reference/config-api/apiserver-audit.v1/#audit-k8s-io-v1-Policy)
resource types by reading the Audit configuration reference.