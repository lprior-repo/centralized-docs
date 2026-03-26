---
doc_id: tutorial/docs-tasks-extend-kubernetes-configure-aggregation-layer.md/docs-tasks-extend-kubernetes-configure-aggregation-layer
chunk_id: tutorial/docs-tasks-extend-kubernetes-configure-aggregation-layer.md/docs-tasks-extend-kubernetes-configure-aggregation-layer#3-detailed
chunk_level: detailed
chunk_type: prose
heading: Authentication Flow
token_count: 886
summary: ### Kubernetes Apiserver Authentication and Authorization A request to an API path that is served by an extension apiserver begins the same way as all API requests: communication to the Kubernetes...
---

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
### Kubernetes Apiserver Proxies the Request
The Kubernetes apiserver now will send, or proxy, the request to the extension
apiserver that registered to handle the request. In order to do so,
it needs to know several things:
1. How should the Kubernetes apiserver authenticate to the extension apiserver,
informing the extension apiserver that the request, which comes over the network,
is coming from a valid Kubernetes apiserver?
2. How should the Kubernetes apiserver inform the extension apiserver of the
username and group for which the original request was authenticated?
In order to provide for these two, you must configure the Kubernetes apiserver using several flags.
#### Kubernetes Apiserver Client Authentication
The Kubernetes apiserver connects to the extension apiserver over TLS,
authenticating itself using a client certificate. You must provide the
following to the Kubernetes apiserver upon startup, using the provided flags:
* private key file via `--proxy-client-key-file`
* signed client certificate file via `--proxy-client-cert-file`
* certificate of the CA that signed the client certificate file via `--requestheader-client-ca-file`
* valid Common Name values (CNs) in the signed client certificate via `--requestheader-allowed-names`
The Kubernetes apiserver will use the files indicated by `--proxy-client-\*-file`
to authenticate to the extension apiserver. In order for the request to be considered
valid by a compliant extension apiserver, the following conditions must be met:
1. The connection must be made using a client certificate that is signed by
the CA whose certificate is in `--requestheader-client-ca-file`.
2. The connection must be made using a client certificate whose CN is one of
those listed in `--requestheader-allowed-names`.
#### Note:
You can set this option to blank as `--requestheader-allowed-names=""`.
This will indicate to an extension apiserver that *any* CN is acceptable.
When started with these options, the Kubernetes apiserver will:
1. Use them to authenticate to the extension apiserver.
2. Create a configmap in the `kube-system` namespace called `extension-apiserver-authentication`,
in which it will place the CA certificate and the allowed CNs. These in turn can be retrieved
by extension apiservers to validate requests.
Note that the same client certificate is used by the Kubernetes apiserver to authenticate
against *all* extension apiservers. It does not create a client certificate per extension
apiserver, but rather a single one to authenticate as the Kubernetes apiserver.
This same one is reused for all extension apiserver requests.
#### Original Request Username and Group
When the Kubernetes apiserver proxies the request to the extension apiserver,
it informs the extension apiserver of the username and group with which the
original request successfully authenticated. It provides these in http headers
of its proxied request. You must inform the Kubernetes apiserver of the names
of the headers to be used.
* the header in which to store the username via `--requestheader-username-headers`
* the header in which to store the group via `--requestheader-group-headers`
* the prefix to append to all extra headers via `--requestheader-extra-headers-prefix`
These header names are also placed in the `extension-apiserver-authentication` configmap,
so they can be retrieved and used by extension apiservers.