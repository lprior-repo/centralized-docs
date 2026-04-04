---
doc_id: ref/docs-tasks-administer-cluster-manage-resources.md/docs-tasks-administer-cluster-manage-resources
chunk_id: ref/docs-tasks-administer-cluster-manage-resources.md/docs-tasks-administer-cluster-manage-resources#0-standard
chunk_level: standard
chunk_type: prose
heading: Table of Contents
token_count: 510
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