---
doc_id: tutorial/docs-tasks-extend-kubernetes-configure-aggregation-layer.md/docs-tasks-extend-kubernetes-configure-aggregation-layer
chunk_id: tutorial/docs-tasks-extend-kubernetes-configure-aggregation-layer.md/docs-tasks-extend-kubernetes-configure-aggregation-layer#2-standard
chunk_level: standard
chunk_type: prose
heading: Authentication Flow
token_count: 439
summary: ## Authentication Flow Unlike Custom Resource Definitions (CRDs), the Aggregation API involves another server - your Extension apiserver - in addition to the standard Kubernetes apiserver. The...
---

## Authentication Flow
Unlike Custom Resource Definitions (CRDs), the Aggregation API involves
another server - your Extension apiserver - in addition to the standard Kubernetes apiserver.
The Kubernetes apiserver will need to communicate with your extension apiserver,
and your extension apiserver will need to communicate with the Kubernetes apiserver.
In order for this communication to be secured, the Kubernetes apiserver uses x509
certificates to authenticate itself to the extension apiserver.
This section describes how the authentication and authorization flows work,
and how to configure them.
The high-level flow is as follows:
1. Kubernetes apiserver: authenticate the requesting user and authorize their
rights to the requested API path.
2. Kubernetes apiserver: proxy the request to the extension apiserver
3. Extension apiserver: authenticate the request from the Kubernetes apiserver
4. Extension apiserver: authorize the request from the original user
5. Extension apiserver: execute
The rest of this section describes these steps in detail.
The flow can be seen in the following diagram.
![aggregation auth flows](/images/docs/aggregation-api-auth-flow.png)
The source for the above swimlanes can be found in the source of this document.
### Kubernetes Apiserver Authentication and Authorization
A request to an API path that is served by an extension apiserver begins
the same way as all API requests: communication to the Kubernetes apiserver.
This path already has been registered with the Kubernetes apiserver by the extension apiserver.
The user communicates with the Kubernetes apiserver, requesting access to the path.
The Kubernetes apiserver uses standard authentication and authorization configured
with the Kubernetes apiserver to authenticate the user and authorize access to the specific path.
For an overview of authenticating to a Kubernetes cluster, see
["Authenticating to a Cluster"](/docs/reference/access-authn-authz/authentication/).
For an overview of authorization of access to Kubernetes cluster resources, see
["Authorization Overview"](/docs/reference/access-authn-authz/authorization/).
Everything to this point has been standard Kubernetes API requests, authentication and authorization.
The Kubernetes apiserver now is prepared to send the request to the extension apiserver.