---
doc_id: ref/docs-concepts-scheduling-eviction-pod-priority-preemption.md/docs-concepts-scheduling-eviction-pod-priority-preemption
chunk_id: ref/docs-concepts-scheduling-eviction-pod-priority-preemption.md/docs-concepts-scheduling-eviction-pod-priority-preemption#6-detailed
chunk_level: detailed
chunk_type: prose
heading: Related Pages
token_count: 733
summary: ## Interactions between Pod priority and quality of service Pod priority and [QoS class](/docs/concepts/workloads/pods/pod-qos/) are two orthogonal features with few interactions and no default...
---

## Interactions between Pod priority and quality of service
Pod priority and [QoS class](/docs/concepts/workloads/pods/pod-qos/)
are two orthogonal features with few interactions and no default restrictions on
setting the priority of a Pod based on its QoS classes. The scheduler's
preemption logic does not consider QoS when choosing preemption targets.
Preemption considers Pod priority and attempts to choose a set of targets with
the lowest priority. Higher-priority Pods are considered for preemption only if
the removal of the lowest priority Pods is not sufficient to allow the scheduler
to schedule the preemptor Pod, or if the lowest priority Pods are protected by
`PodDisruptionBudget`.
The kubelet uses Priority to determine pod order for [node-pressure eviction](/docs/concepts/scheduling-eviction/node-pressure-eviction/).
You can use the QoS class to estimate the order in which pods are most likely
to get evicted. The kubelet ranks pods for eviction based on the following factors:
1. Whether the starved resource usage exceeds requests
2. Pod Priority
3. Amount of resource usage relative to requests
See [Pod selection for kubelet eviction](/docs/concepts/scheduling-eviction/node-pressure-eviction/#pod-selection-for-kubelet-eviction)
for more details.
kubelet node-pressure eviction does not evict Pods when their
usage does not exceed their requests. If a Pod with lower priority is not
exceeding its requests, it won't be evicted. Another Pod with higher priority
that exceeds its requests may be evicted.
## What's next
* Read about using ResourceQuotas in connection with PriorityClasses: [limit Priority Class consumption by default](/docs/concepts/policy/resource-quotas/#limit-priority-class-consumption-by-default)
* Learn about [Pod Disruption](/docs/concepts/workloads/pods/disruptions/)
* Learn about [API-initiated Eviction](/docs/concepts/scheduling-eviction/api-eviction/)
* Learn about [Node-pressure Eviction](/docs/concepts/scheduling-eviction/node-pressure-eviction/)
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

- [Process ID Limits And Reservations](docs-concepts-policy-pid-limiting.md)
- [EndpointSlices](docs-concepts-services-networking-endpoint-slices.md)
- [Secrets](docs-concepts-configuration-secret.md)
- [Example: Deploying Cassandra with a StatefulSet](docs-tutorials-stateful-application-cassandra.md)
- [Configure Quality of Service for Pods](docs-tasks-configure-pod-container-quality-service-pod.md)