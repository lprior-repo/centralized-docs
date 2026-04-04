---
doc_id: tutorial/docs-tasks-administer-cluster-securing-a-cluster.md/docs-tasks-administer-cluster-securing-a-cluster
chunk_id: tutorial/docs-tasks-administer-cluster-securing-a-cluster.md/docs-tasks-administer-cluster-securing-a-cluster#31-summary
chunk_level: summary
chunk_type: prose
heading: Controlling the capabilities of a workload or user at runtime
token_count: 128
summary: to enforce use of a particular [Pod Security Standard](/docs/concepts/security/pod-security-standards/) in a [namespace](/docs/concepts/overview/working-with-objects/namespaces), or to detect...
---

to enforce use of a particular [Pod Security Standard](/docs/concepts/security/pod-security-standards/)
in a [namespace](/docs/concepts/overview/working-with-objects/namespaces), or to detect breaches.
Generally, most application workloads need limited access to host resources so they can
successfully run as a root process (uid 0) without access to host information. However,
considering the privileges associated with the root user, you should write application
containers to run as a non-root user. Similarly, administrators who wish to prevent
client applications from escaping their containers should apply the **Baseline**
or