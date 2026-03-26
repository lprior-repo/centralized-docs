---
doc_id: tutorial/docs-tasks-access-application-cluster-web-ui-dashboard.md/docs-tasks-access-application-cluster-web-ui-dashboard
chunk_id: tutorial/docs-tasks-access-application-cluster-web-ui-dashboard.md/docs-tasks-access-application-cluster-web-ui-dashboard#4-detailed
chunk_level: detailed
chunk_type: prose
heading: Deploying containerized applications
token_count: 915
summary: #### Note: For external Services, you may need to open up one or more ports to do so. Other Services that are only visible from inside the cluster are called internal Services. Irrespective of the...
---

#### Note:
For external Services, you may need to open up one or more ports to do so.
Other Services that are only visible from inside the cluster are called internal Services.
Irrespective of the Service type, if you choose to create a Service and your container listens
on a port (incoming), you need to specify two ports.
The Service will be created mapping the port (incoming) to the target port seen by the container.
This Service will route to your deployed Pods. Supported protocols are TCP and UDP.
The internal DNS name for this Service will be the value you specified as application name above.
If needed, you can expand the **Advanced options** section where you can specify more settings:
* **Description**: The text you enter here will be added as an
[annotation](/docs/concepts/overview/working-with-objects/annotations/)
to the Deployment and displayed in the application's details.
* **Labels**: Default [labels](/docs/concepts/overview/working-with-objects/labels/) to be used
for your application are application name and version.
You can specify additional labels to be applied to the Deployment, Service (if any), and Pods,
such as release, environment, tier, partition, and release track.
Example:
```
`release=1.0
tier=frontend
environment=pod
track=stable
`
```
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