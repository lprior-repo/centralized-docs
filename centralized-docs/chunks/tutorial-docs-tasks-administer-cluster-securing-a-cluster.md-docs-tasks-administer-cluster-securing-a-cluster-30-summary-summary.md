---
doc_id: tutorial/docs-tasks-administer-cluster-securing-a-cluster.md/docs-tasks-administer-cluster-securing-a-cluster
chunk_id: tutorial/docs-tasks-administer-cluster-securing-a-cluster.md/docs-tasks-administer-cluster-securing-a-cluster#30-summary
chunk_level: summary
chunk_type: prose
heading: Controlling the capabilities of a workload or user at runtime
token_count: 113
summary: A pod definition contains a [security context](/docs/tasks/configure-pod-container/security-context/) that allows it to request access to run as a specific Linux user on a node (like root), access to...
---

A pod definition contains a [security context](/docs/tasks/configure-pod-container/security-context/)
that allows it to request access to run as a specific Linux user on a node (like root),
access to run privileged or access the host network, and other controls that would otherwise
allow it to run unfettered on a hosting node.
You can configure [Pod security admission](/docs/concepts/security/pod-security-admission/)
to enforce use of a particular [Pod Security Standard](/docs/concepts/security/pod-security-standards/)
in a