---
doc_id: ref/docs-tasks-administer-cluster-manage-resources.md/docs-tasks-administer-cluster-manage-resources
chunk_id: ref/docs-tasks-administer-cluster-manage-resources.md/docs-tasks-administer-cluster-manage-resources#0-detailed
chunk_level: detailed
chunk_type: prose
heading: Related Pages
token_count: 934
summary: ## Table of Contents          - [[Configure Default Memory Requests and Limits for a...
---

## Table of Contents

        - [[Configure Default Memory Requests and Limits for a Namespace](/docs/tasks/administer-cluster/manage-resources/memory-default-namespace/)](#configure-default-memory-requests-and-limits-for-a-namespacedocstasksadminister-clustermanage-resourcesmemory-default-namespace)
        - [[Configure Default CPU Requests and Limits for a Namespace](/docs/tasks/administer-cluster/manage-resources/cpu-default-namespace/)](#configure-default-cpu-requests-and-limits-for-a-namespacedocstasksadminister-clustermanage-resourcescpu-default-namespace)
        - [[Configure Minimum and Maximum Memory Constraints for a Namespace](/docs/tasks/administer-cluster/manage-resources/memory-constraint-namespace/)](#configure-minimum-and-maximum-memory-constraints-for-a-namespacedocstasksadminister-clustermanage-resourcesmemory-constraint-namespace)
        - [[Configure Minimum and Maximum CPU Constraints for a Namespace](/docs/tasks/administer-cluster/manage-resources/cpu-constraint-namespace/)](#configure-minimum-and-maximum-cpu-constraints-for-a-namespacedocstasksadminister-clustermanage-resourcescpu-constraint-namespace)
        - [[Configure Memory and CPU Quotas for a Namespace](/docs/tasks/administer-cluster/manage-resources/quota-memory-cpu-namespace/)](#configure-memory-and-cpu-quotas-for-a-namespacedocstasksadminister-clustermanage-resourcesquota-memory-cpu-namespace)
        - [[Configure a Pod Quota for a Namespace](/docs/tasks/administer-cluster/manage-resources/quota-pod-namespace/)](#configure-a-pod-quota-for-a-namespacedocstasksadminister-clustermanage-resourcesquota-pod-namespace)
  - [Feedback](#feedback)

---

##### [Configure Default Memory Requests and Limits for a Namespace](/docs/tasks/administer-cluster/manage-resources/memory-default-namespace/)
Define a default memory resource limit for a namespace, so that every new Pod in that namespace has a memory resource limit configured.
##### [Configure Default CPU Requests and Limits for a Namespace](/docs/tasks/administer-cluster/manage-resources/cpu-default-namespace/)
Define a default CPU resource limits for a namespace, so that every new Pod in that namespace has a CPU resource limit configured.
##### [Configure Minimum and Maximum Memory Constraints for a Namespace](/docs/tasks/administer-cluster/manage-resources/memory-constraint-namespace/)
Define a range of valid memory resource limits for a namespace, so that every new Pod in that namespace falls within the range you configure.
##### [Configure Minimum and Maximum CPU Constraints for a Namespace](/docs/tasks/administer-cluster/manage-resources/cpu-constraint-namespace/)
Define a range of valid CPU resource limits for a namespace, so that every new Pod in that namespace falls within the range you configure.
##### [Configure Memory and CPU Quotas for a Namespace](/docs/tasks/administer-cluster/manage-resources/quota-memory-cpu-namespace/)
Define overall memory and CPU resource limits for a namespace.
##### [Configure a Pod Quota for a Namespace](/docs/tasks/administer-cluster/manage-resources/quota-pod-namespace/)
Restrict how many Pods you can create within a namespace.
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
Last modified January 11, 2023 at 11:12 AM PST: [Update page weights in /tasks/administer-cluster section (b1202c78ff)](https://github.com/kubernetes/website/commit/b1202c78ff58867d67c2fb13f1c13e37d8857a28)
## Related Pages

- [Securing a Cluster](docs-tasks-administer-cluster-securing-a-cluster.md)
- [Binding](docs-reference-kubernetes-api-workload-resources-binding-v1.md)
- [conventions](docs-reference-kubectl-conventions.md)
- [HorizontalPodAutoscaler](docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md)
- [Концепции](ru-docs-concepts.md)