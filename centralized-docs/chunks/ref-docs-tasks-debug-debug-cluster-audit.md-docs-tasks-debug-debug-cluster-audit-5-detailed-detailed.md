---
doc_id: ref/docs-tasks-debug-debug-cluster-audit.md/docs-tasks-debug-debug-cluster-audit
chunk_id: ref/docs-tasks-debug-debug-cluster-audit.md/docs-tasks-debug-debug-cluster-audit#5-detailed
chunk_level: detailed
chunk_type: prose
heading: Related Pages
token_count: 795
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
## Feedback
Was this page helpful?
Yes
No
Thanks for the feedback. If you have a specific, answerable question about how to use Kubernetes, ask it on
[Stack Overflow](https://stackoverflow.com/questions/tagged/kubernetes).
Open an issue in the [GitHub Repository](https://www.github.com/kubernetes/website/) if you want to
[report a problem](<https://github.com/kubernetes/website/issues/new?title=Issue with k8s.io>)
or
[suggest an improvement](<https://github.com/kubernetes/website/issues/new?title=Improvement for k8s.io>).
Last modified March 16, 2025 at 5:03 PM PST: [use tabs to list flags specific to each backend in a separate tab. (cc8fd8152a)](https://github.com/kubernetes/website/commit/cc8fd8152a788971ab7dbceb477cc4a1a25733bc)
## Related Pages

- [install kubectl macos](docs-tasks-tools-install-kubectl-macos.md)
- [and then append (or prepend) \~/.local/bin to $PATH](docs-tasks-tools-install-kubectl-linux.md)
- [Binding](docs-reference-kubernetes-api-workload-resources-binding-v1.md)
- [conventions](docs-reference-kubectl-conventions.md)
- [HorizontalPodAutoscaler](docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md)