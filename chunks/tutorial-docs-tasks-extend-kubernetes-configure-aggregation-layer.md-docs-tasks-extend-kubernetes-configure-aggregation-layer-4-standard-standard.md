---
doc_id: tutorial/docs-tasks-extend-kubernetes-configure-aggregation-layer.md/docs-tasks-extend-kubernetes-configure-aggregation-layer
chunk_id: tutorial/docs-tasks-extend-kubernetes-configure-aggregation-layer.md/docs-tasks-extend-kubernetes-configure-aggregation-layer#4-standard
chunk_level: standard
chunk_type: prose
heading: Authentication Flow
token_count: 332
summary: #### Note: You can set this option to blank as `--requestheader-allowed-names=\"\"`. This will indicate to an extension apiserver that *any* CN is acceptable. When started with these options, the...
---

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