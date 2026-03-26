---
doc_id: ref/docs-tasks-debug-debug-application-debug-pods.md/docs-tasks-debug-debug-application-debug-pods
chunk_id: ref/docs-tasks-debug-debug-application-debug-pods.md/docs-tasks-debug-debug-application-debug-pods#2-detailed
chunk_level: detailed
chunk_type: prose
heading: Diagnosing the problem
token_count: 844
summary: ## Diagnosing the problem The first step in troubleshooting is triage. What is the problem? Is it your Pods, your Replication Controller or your Service? * [Debugging Pods](#debugging-pods) *...
---

## Diagnosing the problem
The first step in troubleshooting is triage. What is the problem?
Is it your Pods, your Replication Controller or your Service?
* [Debugging Pods](#debugging-pods)
* [Debugging Replication Controllers](#debugging-replication-controllers)
* [Debugging Services](#debugging-services)### Debugging Pods
The first step in debugging a Pod is taking a look at it. Check the current
state of the Pod and recent events with the following command:
```
`kubectl describe pods ${POD\_NAME}
`
```
Look at the state of the containers in the pod. Are they all `Running`?
Have there been recent restarts?
Continue debugging depending on the state of the pods.
#### My pod stays pending
If a Pod is stuck in `Pending` it means that it can not be scheduled onto a node.
Generally this is because there are insufficient resources of one type or another
that prevent scheduling. Look at the output of the `kubectl describe ...` command above.
There should be messages from the scheduler about why it can not schedule your pod.
Reasons include:
* **You don't have enough resources**: You may have exhausted the supply of CPU
or Memory in your cluster, in this case you need to delete Pods, adjust resource
requests, or add new nodes to your cluster. See [Compute Resources document](/docs/concepts/configuration/manage-resources-containers/)
for more information.
* **You are using `hostPort`**: When you bind a Pod to a `hostPort` there are a
limited number of places that pod can be scheduled. In most cases, `hostPort`
is unnecessary, try using a Service object to expose your Pod. If you do require
`hostPort` then you can only schedule as many Pods as there are nodes in your Kubernetes cluster.
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