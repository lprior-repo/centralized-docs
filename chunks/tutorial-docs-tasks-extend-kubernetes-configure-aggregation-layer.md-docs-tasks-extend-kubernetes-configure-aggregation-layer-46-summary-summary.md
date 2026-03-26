---
doc_id: tutorial/docs-tasks-extend-kubernetes-configure-aggregation-layer.md/docs-tasks-extend-kubernetes-configure-aggregation-layer
chunk_id: tutorial/docs-tasks-extend-kubernetes-configure-aggregation-layer.md/docs-tasks-extend-kubernetes-configure-aggregation-layer#46-summary
chunk_level: summary
chunk_type: prose
heading: Enable Kubernetes Apiserver flags
token_count: 119
summary: #### Contacting the extension apiserver Once the Kubernetes apiserver has determined a request should be sent to an extension apiserver, it needs to know how to contact it. The `service` stanza is a...
---

#### Contacting the extension apiserver
Once the Kubernetes apiserver has determined a request should be sent to an extension apiserver,
it needs to know how to contact it.
The `service` stanza is a reference to the service for an extension apiserver.
The service namespace and name are required. The port is optional and defaults to 443.
Here is an example of an extension apiserver that is configured to be called on port "1234",
and to verify the TLS connection against the ServerName
`my-service-name.my-service-namespace.svc` using a custom CA bundle.