---
doc_id: tutorial/docs-tasks-administer-cluster-securing-a-cluster.md/docs-tasks-administer-cluster-securing-a-cluster
chunk_id: tutorial/docs-tasks-administer-cluster-securing-a-cluster.md/docs-tasks-administer-cluster-securing-a-cluster#56-summary
chunk_level: summary
chunk_type: prose
heading: Protecting cluster components from compromise
token_count: 87
summary: `Secret` or `ConfigMap` objects), the API server writes an encrypted representation of the object. That encryption means that even someone who has access to etcd backup data is unable to view the...
---

`Secret` or
`ConfigMap` objects), the API server writes an encrypted representation of the object.
That encryption means that even someone who has access to etcd backup data is unable
to view the content of those objects.
In Kubernetes 1.35 you can also encrypt custom resources;
encryption-at-rest for extension APIs defined in CustomResourceDefinitions was added to
Kubernetes as part of the v1.26 release.