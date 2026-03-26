---
doc_id: tutorial/docs-tasks-access-application-cluster-web-ui-dashboard.md/docs-tasks-access-application-cluster-web-ui-dashboard
chunk_id: tutorial/docs-tasks-access-application-cluster-web-ui-dashboard.md/docs-tasks-access-application-cluster-web-ui-dashboard#27-summary
chunk_level: summary
chunk_type: prose
heading: Deploying containerized applications
token_count: 118
summary: * **Run command** and **Run command arguments**: By default, your containers run the specified Docker image's default [entrypoint...
---

* **Run command** and **Run command arguments**:
By default, your containers run the specified Docker image's default
[entrypoint command](/docs/tasks/inject-data-application/define-command-argument-container/).
You can use the command options and arguments to override the default.
* **Run as privileged**: This setting determines whether processes in
[privileged containers](/docs/concepts/workloads/pods/#privileged-mode-for-containers)
are equivalent to processes running as root on the host.
Privileged containers can make use of capabilities like manipulating the network stack and accessing devices.