---
doc_id: tutorial/docs-tasks-administer-cluster-securing-a-cluster.md/docs-tasks-administer-cluster-securing-a-cluster
chunk_id: tutorial/docs-tasks-administer-cluster-securing-a-cluster.md/docs-tasks-administer-cluster-securing-a-cluster#53-summary
chunk_level: summary
chunk_type: prose
heading: Protecting cluster components from compromise
token_count: 42
summary: You should not allow untrusted components to create Pods in any system namespace (those with names that start with `kube-`) nor in any namespace where that access grant allows the possibility of...
---

You should not allow untrusted components to create Pods in any system namespace (those with
names that start with `kube-`) nor in any namespace where that access grant allows the possibility
of privilege escalation.