---
doc_id: tutorial/docs-tasks-extend-kubernetes-configure-aggregation-layer.md/docs-tasks-extend-kubernetes-configure-aggregation-layer
chunk_id: tutorial/docs-tasks-extend-kubernetes-configure-aggregation-layer.md/docs-tasks-extend-kubernetes-configure-aggregation-layer#5-standard
chunk_level: standard
chunk_type: prose
heading: Authentication Flow
token_count: 456
summary: ### Extension Apiserver Authenticates the Request The extension apiserver, upon receiving a proxied request from the Kubernetes apiserver, must validate that the request actually did come from a...
---

### Extension Apiserver Authenticates the Request
The extension apiserver, upon receiving a proxied request from the Kubernetes apiserver,
must validate that the request actually did come from a valid authenticating proxy,
which role the Kubernetes apiserver is fulfilling. The extension apiserver validates it via:
1. Retrieve the following from the configmap in `kube-system`, as described above:
* Client CA certificate
* List of allowed names (CNs)
* Header names for username, group and extra info
* Check that the TLS connection was authenticated using a client certificate which:
* Was signed by the CA whose certificate matches the retrieved CA certificate.
* Has a CN in the list of allowed CNs, unless the list is blank, in which case all CNs are allowed.
* Extract the username and group from the appropriate headers
If the above passes, then the request is a valid proxied request from a legitimate
authenticating proxy, in this case the Kubernetes apiserver.
Note that it is the responsibility of the extension apiserver implementation to provide
the above. Many do it by default, leveraging the `k8s.io/apiserver/` package.
Others may provide options to override it using command-line options.
In order to have permission to retrieve the configmap, an extension apiserver
requires the appropriate role. There is a default role named `extension-apiserver-authentication-reader`
in the `kube-system` namespace which can be assigned.
### Extension Apiserver Authorizes the Request
The extension apiserver now can validate that the user/group retrieved from
the headers are authorized to execute the given request. It does so by sending
a standard [SubjectAccessReview](/docs/reference/access-authn-authz/authorization/)
request to the Kubernetes apiserver.
In order for the extension apiserver to be authorized itself to submit the
`SubjectAccessReview` request to the Kubernetes apiserver, it needs the correct permissions.
Kubernetes includes a default `ClusterRole` named `system:auth-delegator` that
has the appropriate permissions. It can be granted to the extension apiserver's service account.
### Extension Apiserver Executes
If the `SubjectAccessReview` passes, the extension apiserver executes the request.