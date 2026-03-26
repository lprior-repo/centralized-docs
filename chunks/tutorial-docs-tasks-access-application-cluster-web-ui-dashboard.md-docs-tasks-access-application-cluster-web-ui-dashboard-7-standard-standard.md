---
doc_id: tutorial/docs-tasks-access-application-cluster-web-ui-dashboard.md/docs-tasks-access-application-cluster-web-ui-dashboard
chunk_id: tutorial/docs-tasks-access-application-cluster-web-ui-dashboard.md/docs-tasks-access-application-cluster-web-ui-dashboard#7-standard
chunk_level: standard
chunk_type: prose
heading: Deploying containerized applications
token_count: 217
summary: * **Run as privileged**: This setting determines whether processes in [privileged containers](/docs/concepts/workloads/pods/#privileged-mode-for-containers) are equivalent to processes running as...
---

* **Run as privileged**: This setting determines whether processes in
[privileged containers](/docs/concepts/workloads/pods/#privileged-mode-for-containers)
are equivalent to processes running as root on the host.
Privileged containers can make use of capabilities like manipulating the network stack and accessing devices.
* **Environment variables**: Kubernetes exposes Services through
[environment variables](/docs/tasks/inject-data-application/environment-variable-expose-pod-information/).
You can compose environment variable or pass arguments to your commands using the values of environment variables.
They can be used in applications to find a Service.
Values can reference other variables using the `$(VAR\_NAME)` syntax.
### Uploading a YAML or JSON file
Kubernetes supports declarative configuration.
In this style, all configuration is stored in manifests (YAML or JSON configuration files).
The manifests use Kubernetes [API](/docs/concepts/overview/kubernetes-api/) resource schemas.
As an alternative to specifying application details in the deploy wizard,
you can define your application in one or more manifests, and upload the files using Dashboard.