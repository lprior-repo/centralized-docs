---
doc_id: tutorial/docs-tutorials-configuration-pod-sidecar-containers.md/docs-tutorials-configuration-pod-sidecar-containers
chunk_id: tutorial/docs-tutorials-configuration-pod-sidecar-containers.md/docs-tutorials-configuration-pod-sidecar-containers#12-summary
chunk_level: summary
chunk_type: prose
heading: Sidecar containers overview
token_count: 123
summary: Sidecar containers are secondary containers that run along with the main application container within the same [Pod](/docs/concepts/workloads/pods/). These containers are used to enhance or to extend...
---

Sidecar containers are secondary containers that run along with the main
application container within the same [Pod](/docs/concepts/workloads/pods/).
These containers are used to enhance or to extend the functionality of the primary *app
container* by providing additional services, or functionalities such as logging, monitoring,
security, or data synchronization, without directly altering the primary application code.
You can read more in the [Sidecar containers](/docs/concepts/workloads/pods/sidecar-containers/)
concept page.
The concept of sidecar containers is not new and there are multiple implementations of this concept.