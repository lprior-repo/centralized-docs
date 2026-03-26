---
doc_id: tutorial/docs-tasks-run-application-scale-stateful-set.md/docs-tasks-run-application-scale-stateful-set
chunk_id: tutorial/docs-tasks-run-application-scale-stateful-set.md/docs-tasks-run-application-scale-stateful-set#1-detailed
chunk_level: detailed
chunk_type: prose
heading: Related Pages
token_count: 1013
summary: # Scale a StatefulSet This task shows how to scale a StatefulSet. Scaling a StatefulSet refers to increasing or decreasing the number of replicas. ## Before you begin * StatefulSets are only...
---

# Scale a StatefulSet
This task shows how to scale a StatefulSet. Scaling a StatefulSet refers to
increasing or decreasing the number of replicas.
## Before you begin
* StatefulSets are only available in Kubernetes version 1.5 or later.
To check your version of Kubernetes, run `kubectl version`.
* Not all stateful applications scale nicely. If you are unsure about whether
to scale your StatefulSets, see [StatefulSet concepts](/docs/concepts/workloads/controllers/statefulset/)
or [StatefulSet tutorial](/docs/tutorials/stateful-application/basic-stateful-set/) for further information.
* You should perform scaling only when you are confident that your stateful application
cluster is completely healthy.
### Use kubectl to scale StatefulSets
First, find the StatefulSet you want to scale.
```
`kubectl get statefulsets &lt;stateful-set-name&gt;
`
```
Change the number of replicas of your StatefulSet:
```
`kubectl scale statefulsets &lt;stateful-set-name&gt; --replicas=&lt;new-replicas&gt;
`
```
### Make in-place updates on your StatefulSets
Alternatively, you can do
[in-place updates](/docs/concepts/cluster-administration/manage-deployment/#in-place-updates-of-resources)
on your StatefulSets.
If your StatefulSet was initially created with `kubectl apply`,
update `.spec.replicas` of the StatefulSet manifests, and then do a `kubectl apply`:
```
`kubectl apply -f &lt;stateful-set-file-updated&gt;
`
```
Otherwise, edit that field with `kubectl edit`:
```
`kubectl edit statefulsets &lt;stateful-set-name&gt;
`
```
Or use `kubectl patch`:
```
`kubectl patch statefulsets &lt;stateful-set-name&gt; -p '{"spec":{"replicas":&lt;new-replicas&gt;}}'
`
```
### Scaling down does not work right
You cannot scale down a StatefulSet when any of the stateful Pods it manages is
unhealthy. Scaling down only takes place after those stateful Pods become running and ready.
If spec.replicas &gt; 1, Kubernetes cannot determine the reason for an unhealthy Pod.
It might be the result of a permanent fault or of a transient fault. A transient
fault can be caused by a restart required by upgrading or maintenance.
If the Pod is unhealthy due to a permanent fault, scaling
without correcting the fault may lead to a state where the StatefulSet membership
drops below a certain minimum number of replicas that are needed to function
correctly. This may cause your StatefulSet to become unavailable.
If the Pod is unhealthy due to a transient fault and the Pod might become available again,
the transient error may interfere with your scale-up or scale-down operation. Some distributed
databases have issues when nodes join and leave at the same time. It is better
to reason about scaling operations at the application level in these cases, and
perform scaling only when you are sure that your stateful application cluster is
completely healthy.
## What's next
* Learn more about [deleting a StatefulSet](/docs/tasks/run-application/delete-stateful-set/).
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
Last modified March 14, 2023 at 8:12 PM PST: [Tweak line wrappings in run-application (8b527bab7e)](https://github.com/kubernetes/website/commit/8b527bab7e78782f4effd452ccaed31bd7617c65)
## Related Pages

- [Example: Deploying Cassandra with a StatefulSet](docs-tutorials-stateful-application-cassandra.md)
- [Adding entries to Pod /etc/hosts with HostAliases](docs-tasks-network-customize-hosts-file-for-pods.md)
- [Change the Access Mode of a PersistentVolume to ReadWriteOncePod](docs-tasks-administer-cluster-change-pv-access-mode-readwriteoncepod.md)
- [Configure Quality of Service for Pods](docs-tasks-configure-pod-container-quality-service-pod.md)
- [Configure Certificate Rotation for the Kubelet](docs-tasks-tls-certificate-rotation.md)