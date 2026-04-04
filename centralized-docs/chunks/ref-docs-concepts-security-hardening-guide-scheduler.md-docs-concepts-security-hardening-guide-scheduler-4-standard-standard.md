---
doc_id: ref/docs-concepts-security-hardening-guide-scheduler.md/docs-concepts-security-hardening-guide-scheduler
chunk_id: ref/docs-concepts-security-hardening-guide-scheduler.md/docs-concepts-security-hardening-guide-scheduler#4-standard
chunk_level: standard
chunk_type: prose
heading: Disallow labeling nodes
token_count: 498
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