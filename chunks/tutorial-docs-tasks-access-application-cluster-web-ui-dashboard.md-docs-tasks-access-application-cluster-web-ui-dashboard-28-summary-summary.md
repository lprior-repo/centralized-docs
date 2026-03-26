---
doc_id: tutorial/docs-tasks-access-application-cluster-web-ui-dashboard.md/docs-tasks-access-application-cluster-web-ui-dashboard
chunk_id: tutorial/docs-tasks-access-application-cluster-web-ui-dashboard.md/docs-tasks-access-application-cluster-web-ui-dashboard#28-summary
chunk_level: summary
chunk_type: prose
heading: Deploying containerized applications
token_count: 74
summary: * **Environment variables**: Kubernetes exposes Services through [environment variables](/docs/tasks/inject-data-application/environment-variable-expose-pod-information/). You can compose environment...
---

* **Environment variables**: Kubernetes exposes Services through
[environment variables](/docs/tasks/inject-data-application/environment-variable-expose-pod-information/).
You can compose environment variable or pass arguments to your commands using the values of environment variables.
They can be used in applications to find a Service.
Values can reference other variables using the `$(VAR\_NAME)` syntax.