---
doc_id: tutorial/docs-tasks-extend-kubernetes-configure-aggregation-layer.md/docs-tasks-extend-kubernetes-configure-aggregation-layer
chunk_id: tutorial/docs-tasks-extend-kubernetes-configure-aggregation-layer.md/docs-tasks-extend-kubernetes-configure-aggregation-layer#27-summary
chunk_level: summary
chunk_type: prose
heading: Authentication Flow
token_count: 127
summary: * Extract the username and group from the appropriate headers If the above passes, then the request is a valid proxied request from a legitimate authenticating proxy, in this case the Kubernetes...
---

* Extract the username and group from the appropriate headers
If the above passes, then the request is a valid proxied request from a legitimate
authenticating proxy, in this case the Kubernetes apiserver.
Note that it is the responsibility of the extension apiserver implementation to provide
the above. Many do it by default, leveraging the `k8s.io/apiserver/` package.
Others may provide options to override it using command-line options.
In order to have permission to retrieve the configmap, an extension apiserver
requires the appropriate role. There is a default role named `extension-apiserver-authentication-reader`