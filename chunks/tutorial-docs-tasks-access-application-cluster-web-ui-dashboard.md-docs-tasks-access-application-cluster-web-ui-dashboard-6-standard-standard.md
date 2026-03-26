---
doc_id: tutorial/docs-tasks-access-application-cluster-web-ui-dashboard.md/docs-tasks-access-application-cluster-web-ui-dashboard
chunk_id: tutorial/docs-tasks-access-application-cluster-web-ui-dashboard.md/docs-tasks-access-application-cluster-web-ui-dashboard#6-standard
chunk_level: standard
chunk_type: prose
heading: Deploying containerized applications
token_count: 478
summary: * **Namespace**: Kubernetes supports multiple virtual clusters backed by the same physical cluster. These virtual clusters are called [namespaces](/docs/tasks/administer-cluster/namespaces/). They...
---

* **Namespace**: Kubernetes supports multiple virtual clusters backed by the same physical cluster.
These virtual clusters are called [namespaces](/docs/tasks/administer-cluster/namespaces/).
They let you partition resources into logically named groups.
Dashboard offers all available namespaces in a dropdown list, and allows you to create a new namespace.
The namespace name may contain a maximum of 63 alphanumeric characters and dashes (-) but can not contain capital letters.
Namespace names should not consist of only numbers.
If the name is set as a number, such as 10, the pod will be put in the default namespace.
In case the creation of the namespace is successful, it is selected by default.
If the creation fails, the first namespace is selected.
* **Image Pull Secret**:
In case the specified Docker container image is private, it may require
[pull secret](/docs/concepts/configuration/secret/) credentials.
Dashboard offers all available secrets in a dropdown list, and allows you to create a new secret.
The secret name must follow the DNS domain name syntax, for example `new.image-pull.secret`.
The content of a secret must be base64-encoded and specified in a
[`.dockercfg`](/docs/concepts/containers/images/#specifying-imagepullsecrets-on-a-pod) file.
The secret name may consist of a maximum of 253 characters.
In case the creation of the image pull secret is successful, it is selected by default. If the creation fails, no secret is applied.
* **CPU requirement (cores)** and **Memory requirement (MiB)**:
You can specify the minimum [resource limits](/docs/tasks/administer-cluster/manage-resources/memory-default-namespace/)
for the container. By default, Pods run with unbounded CPU and memory limits.
* **Run command** and **Run command arguments**:
By default, your containers run the specified Docker image's default
[entrypoint command](/docs/tasks/inject-data-application/define-command-argument-container/).
You can use the command options and arguments to override the default.
* **Run as privileged**: This setting determines whether processes in
[privileged containers](/docs/concepts/workloads/pods/#privileged-mode-for-containers)
are equivalent to processes running as root on the host.
Privileged containers can make use of capabilities like manipulating the network stack and accessing devices.