---
doc_id: ref/docs-concepts-security-hardening-guide-scheduler.md/docs-concepts-security-hardening-guide-scheduler
chunk_id: ref/docs-concepts-security-hardening-guide-scheduler.md/docs-concepts-security-hardening-guide-scheduler#2-detailed
chunk_level: detailed
chunk_type: prose
heading: Related Pages
token_count: 781
summary: ### Key considerations * Exactly one plugin that uses the `queueSort` extension point can be enabled at a time. Any plugins that use `queueSort` should be scrutinized. * Plugins that implement the...
---

### Key considerations
* Exactly one plugin that uses the `queueSort` extension point can be enabled at a time.
Any plugins that use `queueSort` should be scrutinized.
* Plugins that implement the `prefilter` or `filter` extension point can potentially mark all nodes as unschedulable.
This can bring scheduling of new pods to a halt.
* Plugins that implement the `permit` extension point can prevent or delay the binding of a Pod.
Such plugins should be thoroughly reviewed by the cluster administrator.
When using a plugin that is not one of the [default plugins](/docs/reference/scheduling/config/#scheduling-plugins),
consider disabling the `queueSort`, `filter` and `permit` extension points as follows:
```
`apiVersion: kubescheduler.config.k8s.io/v1
kind: KubeSchedulerConfiguration
profiles:
- schedulerName: my-scheduler
plugins:
# You can disable all plugins for an extension point using "\*"
queueSort:
disabled:
- name: "\*" # Disable all queueSort plugins
# - name: "PrioritySort" # Disable specific queueSort plugin
filter:
disabled:
- name: "\*" # Disable all filter plugins
# - name: "NodeResourcesFit" # Disable specific filter plugin
permit:
disabled:
- name: "\*" # Disables all permit plugins
# - name: "TaintToleration" # Disable specific permit plugin
`
```
This creates a scheduler profile `my-scheduler`.
Whenever the `.spec` of a Pod does not have a value for `.spec.schedulerName`, the kube-scheduler runs for that Pod,
using its main configuration, and default plugins.
If you define a Pod with `.spec.schedulerName` set to `my-scheduler`, the kube-scheduler runs
but with a custom configuration; in that custom configuration,
the `queueSort`, `filter` and `permit` extension points are disabled.
If you use this KubeSchedulerConfiguration, and don't run any custom scheduler,
and you then define a Pod with `.spec.schedulerName` set to `nonexistent-scheduler`
(or any other scheduler name that doesn't exist in your cluster), no events would be generated for a pod.
## Disallow labeling nodes
A cluster administrator should ensure that cluster users cannot label the nodes.
A malicious actor can use `nodeSelector` to schedule workloads on nodes where those workloads should not be present.
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
Last modified February 19, 2026 at 3:34 PM PST: [Fix some links in the En docs (95b7685f71)](https://github.com/kubernetes/website/commit/95b7685f7156c317aa59d86618e8ec4535d2015f)
## Related Pages

- [Kubernetes Scheduler](docs-concepts-scheduling-eviction-kube-scheduler.md)
- [Using RBAC Authorization](docs-reference-access-authn-authz-rbac.md)
- [Objects In Kubernetes](docs-concepts-overview-working-with-objects.md)
- [Binding](docs-reference-kubernetes-api-workload-resources-binding-v1.md)
- [conventions](docs-reference-kubectl-conventions.md)