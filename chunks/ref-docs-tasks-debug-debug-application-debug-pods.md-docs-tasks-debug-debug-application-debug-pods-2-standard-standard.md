---
doc_id: ref/docs-tasks-debug-debug-application-debug-pods.md/docs-tasks-debug-debug-application-debug-pods
chunk_id: ref/docs-tasks-debug-debug-application-debug-pods.md/docs-tasks-debug-debug-application-debug-pods#2-standard
chunk_level: standard
chunk_type: prose
heading: Diagnosing the problem
token_count: 386
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