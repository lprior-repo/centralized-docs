---
doc_id: tutorial/docs-tasks-access-application-cluster-web-ui-dashboard.md/docs-tasks-access-application-cluster-web-ui-dashboard
chunk_id: tutorial/docs-tasks-access-application-cluster-web-ui-dashboard.md/docs-tasks-access-application-cluster-web-ui-dashboard#15-summary
chunk_level: summary
chunk_type: prose
heading: Deploying containerized applications
token_count: 120
summary: * **App name** (mandatory): Name for your application. A [label](/docs/concepts/overview/working-with-objects/labels/) with the name will be added to the Deployment and Service, if any, that will be...
---

* **App name** (mandatory): Name for your application.
A [label](/docs/concepts/overview/working-with-objects/labels/) with the name will be
added to the Deployment and Service, if any, that will be deployed.
The application name must be unique within the selected Kubernetes [namespace](/docs/tasks/administer-cluster/namespaces/).
It must start with a lowercase character, and end with a lowercase character or a number,
and contain only lowercase letters, numbers and dashes (-). It is limited to 24 characters.
Leading and trailing spaces are ignored.