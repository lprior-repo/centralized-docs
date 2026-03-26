---
doc_id: ref/docs-tasks-debug-debug-application-debug-pods.md/docs-tasks-debug-debug-application-debug-pods
chunk_id: ref/docs-tasks-debug-debug-application-debug-pods.md/docs-tasks-debug-debug-application-debug-pods#3-standard
chunk_level: standard
chunk_type: prose
heading: Diagnosing the problem
token_count: 458
summary: #### My pod stays waiting If a Pod is stuck in the `Waiting` state, then it has been scheduled to a worker node, but it can't run on that machine. Again, the information from `kubectl describe ...`...
---

#### My pod stays waiting
If a Pod is stuck in the `Waiting` state, then it has been scheduled to a worker node,
but it can't run on that machine. Again, the information from `kubectl describe ...`
should be informative. The most common cause of `Waiting` pods is a failure to pull the image.
There are three things to check:
* Make sure that you have the name of the image correct.
* Have you pushed the image to the registry?
* Try to manually pull the image to see if the image can be pulled. For example,
if you use Docker on your PC, run `docker pull &lt;image&gt;`.#### My pod stays terminating
If a Pod is stuck in the `Terminating` state, it means that a deletion has been
issued for the Pod, but the control plane is unable to delete the Pod object.
This typically happens if the Pod has a [finalizer](/docs/concepts/overview/working-with-objects/finalizers/)
and there is an [admission webhook](/docs/reference/access-authn-authz/extensible-admission-controllers/)
installed in the cluster that prevents the control plane from removing the
finalizer.
To identify this scenario, check if your cluster has any
ValidatingWebhookConfiguration or MutatingWebhookConfiguration that target
`UPDATE` operations for `pods` resources.
If the webhook is provided by a third-party:
* Make sure you are using the latest version.
* Disable the webhook for `UPDATE` operations.
* Report an issue with the corresponding provider.
If you are the author of the webhook:
* For a mutating webhook, make sure it never changes immutable fields on
`UPDATE` operations. For example, changes to containers are usually not allowed.
* For a validating webhook, make sure that your validation policies only apply
to new changes. In other words, you should allow Pods with existing violations
to pass validation. This allows Pods that were created before the validating
webhook was installed to continue running.#### My pod is crashing or otherwise unhealthy
Once your pod has been scheduled, the methods described in
[Debug Running Pods](/docs/tasks/debug/debug-application/debug-running-pod/)
are available for debugging.