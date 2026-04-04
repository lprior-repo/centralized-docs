---
doc_id: ref/docs-tasks-run-application-force-delete-stateful-set-pod.md/docs-tasks-run-application-force-delete-stateful-set-pod
chunk_id: ref/docs-tasks-run-application-force-delete-stateful-set-pod.md/docs-tasks-run-application-force-delete-stateful-set-pod#2-detailed
chunk_level: detailed
chunk_type: prose
heading: Feedback
token_count: 963
summary: ## Delete Pods You can perform a graceful pod deletion with the following command: ``` `kubectl delete pods &lt;pod&gt; ` ``` For the above to lead to graceful termination, the Pod **must not**...
---

## Delete Pods
You can perform a graceful pod deletion with the following command:
```
`kubectl delete pods &lt;pod&gt;
`
```
For the above to lead to graceful termination, the Pod **must not** specify a
`pod.Spec.TerminationGracePeriodSeconds` of 0. The practice of setting a
`pod.Spec.TerminationGracePeriodSeconds` of 0 seconds is unsafe and strongly discouraged
for StatefulSet Pods. Graceful deletion is safe and will ensure that the Pod
[shuts down gracefully](/docs/concepts/workloads/pods/pod-lifecycle/#pod-termination)
before the kubelet deletes the name from the apiserver.
A Pod is not deleted automatically when a node is unreachable.
The Pods running on an unreachable Node enter the 'Terminating' or 'Unknown' state after a
[timeout](/docs/concepts/architecture/nodes/#condition).
Pods may also enter these states when the user attempts graceful deletion of a Pod
on an unreachable Node.
The only ways in which a Pod in such a state can be removed from the apiserver are as follows:
* The Node object is deleted (either by you, or by the
[Node Controller](/docs/concepts/architecture/nodes/#node-controller)).
* The kubelet on the unresponsive Node starts responding, kills the Pod and removes the entry
from the apiserver.
* Force deletion of the Pod by the user.
The recommended best practice is to use the first or second approach. If a Node is confirmed
to be dead (e.g. permanently disconnected from the network, powered down, etc), then delete
the Node object. If the Node is suffering from a network partition, then try to resolve this
or wait for it to resolve. When the partition heals, the kubelet will complete the deletion
of the Pod and free up its name in the apiserver.
Normally, the system completes the deletion once the Pod is no longer running on a Node, or
the Node is deleted by an administrator. You may override this by force deleting the Pod.
### Force Deletion
Force deletions **do not** wait for confirmation from the kubelet that the Pod has been terminated.
Irrespective of whether a force deletion is successful in killing a Pod, it will immediately
free up the name from the apiserver. This would let the StatefulSet controller create a replacement
Pod with that same identity; this can lead to the duplication of a still-running Pod,
and if said Pod can still communicate with the other members of the StatefulSet,
will violate the at most one semantics that StatefulSet is designed to guarantee.
When you force delete a StatefulSet pod, you are asserting that the Pod in question will never
again make contact with other Pods in the StatefulSet and its name can be safely freed up for a
replacement to be created.
If you want to delete a Pod forcibly using kubectl version &gt;= 1.5, do the following:
```
`kubectl delete pods &lt;pod&gt; --grace-period=0 --force
`
```
If you're using any version of kubectl &lt;= 1.4, you should omit the `--force` option and use:
```
`kubectl delete pods &lt;pod&gt; --grace-period=0
`
```
If even after these commands the pod is stuck on `Unknown` state, use the following command to
remove the pod from the cluster:
```
`kubectl patch pod &lt;pod&gt; -p '{"metadata":{"finalizers":null}}'
`
```
Always perform force deletion of StatefulSet Pods carefully and with complete knowledge of the risks involved.
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
Last modified February 19, 2023 at 9:42 PM PST: [Clean up page in tasks/run-application (ba99616c27)](https://github.com/kubernetes/website/commit/ba99616c271e001ef89f4d0bfd9b6c0eaf45e410)