---
doc_id: tutorial/docs-tasks-access-application-cluster-web-ui-dashboard.md/docs-tasks-access-application-cluster-web-ui-dashboard
chunk_id: tutorial/docs-tasks-access-application-cluster-web-ui-dashboard.md/docs-tasks-access-application-cluster-web-ui-dashboard#3-detailed
chunk_level: detailed
chunk_type: prose
heading: Deploying containerized applications
token_count: 310
summary: ### Specifying application details The deploy wizard expects that you provide the following information: * **App name** (mandatory): Name for your application. A...
---

### Specifying application details
The deploy wizard expects that you provide the following information:
* **App name** (mandatory): Name for your application.
A [label](/docs/concepts/overview/working-with-objects/labels/) with the name will be
added to the Deployment and Service, if any, that will be deployed.
The application name must be unique within the selected Kubernetes [namespace](/docs/tasks/administer-cluster/namespaces/).
It must start with a lowercase character, and end with a lowercase character or a number,
and contain only lowercase letters, numbers and dashes (-). It is limited to 24 characters.
Leading and trailing spaces are ignored.
* **Container image** (mandatory):
The URL of a public Docker [container image](/docs/concepts/containers/images/) on any registry,
or a private image (commonly hosted on the Google Container Registry or Docker Hub).
The container image specification must end with a colon.
* **Number of pods** (mandatory): The target number of Pods you want your application to be deployed in.
The value must be a positive integer.
A [Deployment](/docs/concepts/workloads/controllers/deployment/) will be created to
maintain the desired number of Pods across your cluster.
* **Service** (optional): For some parts of your application (e.g. frontends) you may want to expose a
[Service](/docs/concepts/services-networking/service/) onto an external,
maybe public IP address outside of your cluster (external Service).