---
doc_id: ref/docs-tasks-administer-cluster-guaranteed-scheduling-critical-addon-pods.md/docs-tasks-administer-cluster-guaranteed-scheduling-critical-addon-pods
chunk_id: ref/docs-tasks-administer-cluster-guaranteed-scheduling-critical-addon-pods.md/docs-tasks-administer-cluster-guaranteed-scheduling-critical-addon-pods#0-detailed
chunk_level: detailed
chunk_type: prose
heading: Related Pages
token_count: 599
summary: ## Table of Contents  - [Guaranteed Scheduling For Critical Add-On Pods](#guaranteed-scheduling-for-critical-add-on-pods)     - [Marking pod as critical](#marking-pod-as-critical)   -...
---

## Table of Contents

- [Guaranteed Scheduling For Critical Add-On Pods](#guaranteed-scheduling-for-critical-add-on-pods)
    - [Marking pod as critical](#marking-pod-as-critical)
  - [Feedback](#feedback)

---

# Guaranteed Scheduling For Critical Add-On Pods
Kubernetes core components such as the API server, scheduler, and controller-manager run on a control plane node. However, add-ons must run on a regular cluster node.
Some of these add-ons are critical to a fully functional cluster, such as metrics-server, DNS, and UI.
A cluster may stop working properly if a critical add-on is evicted (either manually or as a side effect of another operation like upgrade)
and becomes pending (for example when the cluster is highly utilized and either there are other pending pods that schedule into the space
vacated by the evicted critical add-on pod or the amount of resources available on the node changed for some other reason).
Note that marking a pod as critical is not meant to prevent evictions entirely; it only prevents the pod from becoming permanently unavailable.
A static pod marked as critical can't be evicted. However, non-static pods marked as critical are always rescheduled.
### Marking pod as critical
To mark a Pod as critical, set priorityClassName for that Pod to `system-cluster-critical` or `system-node-critical`. `system-node-critical` is the highest available priority, even higher than `system-cluster-critical`.
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
Last modified January 20, 2025 at 8:58 AM PST: [Drop stale reviewers (58b4f374b8)](https://github.com/kubernetes/website/commit/58b4f374b8f36a73612ef26ba5a19f10a7b0c135)
## Related Pages

- [Binding](docs-reference-kubernetes-api-workload-resources-binding-v1.md)
- [conventions](docs-reference-kubectl-conventions.md)
- [HorizontalPodAutoscaler](docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md)
- [Концепции](ru-docs-concepts.md)
- [Using RBAC Authorization](docs-reference-access-authn-authz-rbac.md)