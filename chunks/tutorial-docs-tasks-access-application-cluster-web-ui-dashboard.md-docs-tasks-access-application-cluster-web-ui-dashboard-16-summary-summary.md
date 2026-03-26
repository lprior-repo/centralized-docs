---
doc_id: tutorial/docs-tasks-access-application-cluster-web-ui-dashboard.md/docs-tasks-access-application-cluster-web-ui-dashboard
chunk_id: tutorial/docs-tasks-access-application-cluster-web-ui-dashboard.md/docs-tasks-access-application-cluster-web-ui-dashboard#16-summary
chunk_level: summary
chunk_type: prose
heading: Deploying containerized applications
token_count: 118
summary: * **Container image** (mandatory): The URL of a public Docker [container image](/docs/concepts/containers/images/) on any registry, or a private image (commonly hosted on the Google Container...
---

* **Container image** (mandatory):
The URL of a public Docker [container image](/docs/concepts/containers/images/) on any registry,
or a private image (commonly hosted on the Google Container Registry or Docker Hub).
The container image specification must end with a colon.
* **Number of pods** (mandatory): The target number of Pods you want your application to be deployed in.
The value must be a positive integer.
A [Deployment](/docs/concepts/workloads/controllers/deployment/) will be created to
maintain the desired number of Pods across your cluster.