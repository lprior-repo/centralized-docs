---
doc_id: tutorial/docs-tasks-access-application-cluster-web-ui-dashboard.md/docs-tasks-access-application-cluster-web-ui-dashboard
chunk_id: tutorial/docs-tasks-access-application-cluster-web-ui-dashboard.md/docs-tasks-access-application-cluster-web-ui-dashboard#26-summary
chunk_level: summary
chunk_type: prose
heading: Deploying containerized applications
token_count: 123
summary: [`.dockercfg`](/docs/concepts/containers/images/#specifying-imagepullsecrets-on-a-pod) file. The secret name may consist of a maximum of 253 characters. In case the creation of the image pull secret...
---

[`.dockercfg`](/docs/concepts/containers/images/#specifying-imagepullsecrets-on-a-pod) file.
The secret name may consist of a maximum of 253 characters.
In case the creation of the image pull secret is successful, it is selected by default. If the creation fails, no secret is applied.
* **CPU requirement (cores)** and **Memory requirement (MiB)**:
You can specify the minimum [resource limits](/docs/tasks/administer-cluster/manage-resources/memory-default-namespace/)
for the container. By default, Pods run with unbounded CPU and memory limits.