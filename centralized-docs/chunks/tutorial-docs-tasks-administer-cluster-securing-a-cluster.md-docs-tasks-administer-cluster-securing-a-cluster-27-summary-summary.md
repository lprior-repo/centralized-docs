---
doc_id: tutorial/docs-tasks-administer-cluster-securing-a-cluster.md/docs-tasks-administer-cluster-securing-a-cluster
chunk_id: tutorial/docs-tasks-administer-cluster-securing-a-cluster.md/docs-tasks-administer-cluster-securing-a-cluster#27-summary
chunk_level: summary
chunk_type: prose
heading: Controlling the capabilities of a workload or user at runtime
token_count: 114
summary: [Resource quota](/docs/concepts/policy/resource-quotas/) limits the number or capacity of resources granted to a namespace. This is most often used to limit the amount of CPU, memory, or persistent...
---

[Resource quota](/docs/concepts/policy/resource-quotas/) limits the number or capacity of
resources granted to a namespace. This is most often used to limit the amount of CPU, memory,
or persistent disk a namespace can allocate, but can also control how many pods, services, or
volumes exist in each namespace.
[Limit ranges](/docs/tasks/administer-cluster/manage-resources/memory-default-namespace/) restrict the maximum or minimum size of some of the
resources above, to prevent users from requesting unreasonably high or low values for commonly