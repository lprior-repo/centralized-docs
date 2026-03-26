---
doc_id: tutorial/docs-tutorials-configuration-pod-sidecar-containers.md/docs-tutorials-configuration-pod-sidecar-containers
chunk_id: tutorial/docs-tutorials-configuration-pod-sidecar-containers.md/docs-tutorials-configuration-pod-sidecar-containers#14-summary
chunk_level: summary
chunk_type: prose
heading: Sidecar containers overview
token_count: 121
summary: sidecars are often [mutating webhooks](/docs/reference/access-authn-authz/admission-controllers/#mutatingadmissionwebhook). For example, a service mesh addon might inject a sidecar that configures...
---

sidecars are often [mutating webhooks](/docs/reference/access-authn-authz/admission-controllers/#mutatingadmissionwebhook).
For example, a service mesh addon might inject a sidecar that configures mutual TLS and encryption
in transit between different Pods.
While the concept of sidecar containers is not new,
the native implementation of this feature in Kubernetes, however, is new. And as with every new feature,
adopting this feature may present certain challenges.
This tutorial explores challenges and solutions that can be experienced by end users as well as
by authors of sidecar containers.