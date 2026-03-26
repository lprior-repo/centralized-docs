---
doc_id: tutorial/docs-tutorials-configuration-pod-sidecar-containers.md/docs-tutorials-configuration-pod-sidecar-containers
chunk_id: tutorial/docs-tutorials-configuration-pod-sidecar-containers.md/docs-tutorials-configuration-pod-sidecar-containers#2-standard
chunk_level: standard
chunk_type: prose
heading: Sidecar containers overview
token_count: 318
summary: ## Sidecar containers overview Sidecar containers are secondary containers that run along with the main application container within the same [Pod](/docs/concepts/workloads/pods/). These containers...
---

## Sidecar containers overview
Sidecar containers are secondary containers that run along with the main
application container within the same [Pod](/docs/concepts/workloads/pods/).
These containers are used to enhance or to extend the functionality of the primary *app
container* by providing additional services, or functionalities such as logging, monitoring,
security, or data synchronization, without directly altering the primary application code.
You can read more in the [Sidecar containers](/docs/concepts/workloads/pods/sidecar-containers/)
concept page.
The concept of sidecar containers is not new and there are multiple implementations of this concept.
As well as sidecar containers that you, the person defining the Pod, want to run, you can also find
that some [addons](/docs/concepts/cluster-administration/addons/) modify Pods - before the Pods
start running - so that there are extra sidecar containers. The mechanisms to *inject* those extra
sidecars are often [mutating webhooks](/docs/reference/access-authn-authz/admission-controllers/#mutatingadmissionwebhook).
For example, a service mesh addon might inject a sidecar that configures mutual TLS and encryption
in transit between different Pods.
While the concept of sidecar containers is not new,
the native implementation of this feature in Kubernetes, however, is new. And as with every new feature,
adopting this feature may present certain challenges.
This tutorial explores challenges and solutions that can be experienced by end users as well as
by authors of sidecar containers.