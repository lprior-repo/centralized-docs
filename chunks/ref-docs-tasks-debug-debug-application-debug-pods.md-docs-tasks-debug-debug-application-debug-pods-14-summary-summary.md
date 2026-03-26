---
doc_id: ref/docs-tasks-debug-debug-application-debug-pods.md/docs-tasks-debug-debug-application-debug-pods
chunk_id: ref/docs-tasks-debug-debug-application-debug-pods.md/docs-tasks-debug-debug-application-debug-pods#14-summary
chunk_level: summary
chunk_type: prose
heading: Diagnosing the problem
token_count: 126
summary: [finalizer](/docs/concepts/overview/working-with-objects/finalizers/) and there is an [admission webhook](/docs/reference/access-authn-authz/extensible-admission-controllers/) installed in the...
---

[finalizer](/docs/concepts/overview/working-with-objects/finalizers/)
and there is an [admission webhook](/docs/reference/access-authn-authz/extensible-admission-controllers/)
installed in the cluster that prevents the control plane from removing the
finalizer.
To identify this scenario, check if your cluster has any
ValidatingWebhookConfiguration or MutatingWebhookConfiguration that target
`UPDATE` operations for `pods` resources.
If the webhook is provided by a third-party:
* Make sure you are using the latest version.
* Disable the webhook for `UPDATE` operations.