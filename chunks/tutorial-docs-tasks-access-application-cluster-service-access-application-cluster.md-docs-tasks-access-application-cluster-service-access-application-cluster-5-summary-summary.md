---
doc_id: tutorial/docs-tasks-access-application-cluster-service-access-application-cluster.md/docs-tasks-access-application-cluster-service-access-application-cluster
chunk_id: tutorial/docs-tasks-access-application-cluster-service-access-application-cluster.md/docs-tasks-access-application-cluster-service-access-application-cluster#5-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 100
summary: * Run two instances of a Hello World application. * Create a Service object that exposes a node port. * Use the Service object to access the running application.## Creating a service for an...
---

* Run two instances of a Hello World application.
* Create a Service object that exposes a node port.
* Use the Service object to access the running application.## Creating a service for an application running in two pods
Here is the configuration file for the application Deployment:
[`service/access/hello-application.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/service/access/hello-application.yaml)![](/images/copycode.svg "Copy service/access/hello-application.yaml to clipboard")