---
doc_id: tutorial/docs-tutorials-configuration-pod-sidecar-containers.md/docs-tutorials-configuration-pod-sidecar-containers
chunk_id: tutorial/docs-tutorials-configuration-pod-sidecar-containers.md/docs-tutorials-configuration-pod-sidecar-containers#13-summary
chunk_level: summary
chunk_type: prose
heading: Sidecar containers overview
token_count: 119
summary: concept page. The concept of sidecar containers is not new and there are multiple implementations of this concept. As well as sidecar containers that you, the person defining the Pod, want to run,...
---

concept page.
The concept of sidecar containers is not new and there are multiple implementations of this concept.
As well as sidecar containers that you, the person defining the Pod, want to run, you can also find
that some [addons](/docs/concepts/cluster-administration/addons/) modify Pods - before the Pods
start running - so that there are extra sidecar containers. The mechanisms to *inject* those extra
sidecars are often [mutating webhooks](/docs/reference/access-authn-authz/admission-controllers/#mutatingadmissionwebhook).