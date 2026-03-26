---
doc_id: tutorial/docs-tasks-access-application-cluster-web-ui-dashboard.md/docs-tasks-access-application-cluster-web-ui-dashboard
chunk_id: tutorial/docs-tasks-access-application-cluster-web-ui-dashboard.md/docs-tasks-access-application-cluster-web-ui-dashboard#23-summary
chunk_level: summary
chunk_type: prose
heading: Deploying containerized applications
token_count: 121
summary: * **Namespace**: Kubernetes supports multiple virtual clusters backed by the same physical cluster. These virtual clusters are called [namespaces](/docs/tasks/administer-cluster/namespaces/). They...
---

* **Namespace**: Kubernetes supports multiple virtual clusters backed by the same physical cluster.
These virtual clusters are called [namespaces](/docs/tasks/administer-cluster/namespaces/).
They let you partition resources into logically named groups.
Dashboard offers all available namespaces in a dropdown list, and allows you to create a new namespace.
The namespace name may contain a maximum of 63 alphanumeric characters and dashes (-) but can not contain capital letters.
Namespace names should not consist of only numbers.
If the name is set as a number, such as 10, the pod will be put in the default namespace.