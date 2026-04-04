---
doc_id: ref/docs-concepts-security-controlling-access.md/docs-concepts-security-controlling-access
chunk_id: ref/docs-concepts-security-controlling-access.md/docs-concepts-security-controlling-access#1-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 124
summary: # Controlling Access to the Kubernetes API This page provides an overview of controlling access to the Kubernetes API. Users access the [Kubernetes API](/docs/concepts/overview/kubernetes-api/) using...
---

# Controlling Access to the Kubernetes API
This page provides an overview of controlling access to the Kubernetes API.
Users access the [Kubernetes API](/docs/concepts/overview/kubernetes-api/) using `kubectl`,
client libraries, or by making REST requests. Both human users and
[Kubernetes service accounts](/docs/tasks/configure-pod-container/configure-service-account/) can be
authorized for API access.
When a request reaches the API, it goes through several stages, illustrated in the
following diagram:
![Diagram of request handling steps for Kubernetes API request](/images/docs/admin/access-control-overview.svg)