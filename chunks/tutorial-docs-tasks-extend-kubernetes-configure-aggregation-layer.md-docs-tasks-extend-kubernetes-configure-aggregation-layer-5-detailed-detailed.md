---
doc_id: tutorial/docs-tasks-extend-kubernetes-configure-aggregation-layer.md/docs-tasks-extend-kubernetes-configure-aggregation-layer
chunk_id: tutorial/docs-tasks-extend-kubernetes-configure-aggregation-layer.md/docs-tasks-extend-kubernetes-configure-aggregation-layer#5-detailed
chunk_level: detailed
chunk_type: prose
heading: Enable Kubernetes Apiserver flags
token_count: 748
summary: ## Enable Kubernetes Apiserver flags Enable the aggregation layer via the following `kube-apiserver` flags. They may have already been taken care of by your provider. ```...
---

## Enable Kubernetes Apiserver flags
Enable the aggregation layer via the following `kube-apiserver` flags.
They may have already been taken care of by your provider.
```
`--requestheader-client-ca-file=&lt;path to aggregator CA cert&gt;
--requestheader-allowed-names=front-proxy-client
--requestheader-extra-headers-prefix=X-Remote-Extra-
--requestheader-group-headers=X-Remote-Group
--requestheader-username-headers=X-Remote-User
--proxy-client-cert-file=&lt;path to aggregator proxy cert&gt;
--proxy-client-key-file=&lt;path to aggregator proxy key&gt;
`
```
### CA Reusage and Conflicts
The Kubernetes apiserver has two client CA options:
* `--client-ca-file`
* `--requestheader-client-ca-file`
Each of these functions independently and can conflict with each other,
if not used correctly.
* `--client-ca-file`: When a request arrives to the Kubernetes apiserver,
if this option is enabled, the Kubernetes apiserver checks the certificate
of the request. If it is signed by one of the CA certificates in the file referenced by
`--client-ca-file`, then the request is treated as a legitimate request,
and the user is the value of the common name `CN=`, while the group is the organization `O=`.
See the [documentation on TLS authentication](/docs/reference/access-authn-authz/authentication/#x509-client-certificates).
* `--requestheader-client-ca-file`: When a request arrives to the Kubernetes apiserver,
if this option is enabled, the Kubernetes apiserver checks the certificate of the request.
If it is signed by one of the CA certificates in the file reference by `--requestheader-client-ca-file`,
then the request is treated as a potentially legitimate request. The Kubernetes apiserver then
checks if the common name `CN=` is one of the names in the list provided by `--requestheader-allowed-names`.
If the name is allowed, the request is approved; if it is not, the request is not.
If *both* `--client-ca-file` and `--requestheader-client-ca-file` are provided,
then the request first checks the `--requestheader-client-ca-file` CA and then the
`--client-ca-file`. Normally, different CAs, either root CAs or intermediate CAs,
are used for each of these options; regular client requests match against `--client-ca-file`,
while aggregation requests match against `--requestheader-client-ca-file`. However,
if both use the *same* CA, then client requests that normally would pass via `--client-ca-file`
will fail, because the CA will match the CA in `--requestheader-client-ca-file`,
but the common name `CN=` will **not** match one of the acceptable common names in
`--requestheader-allowed-names`. This can cause your kubelets and other control plane components,
as well as end-users, to be unable to authenticate to the Kubernetes apiserver.
For this reason, use different CA certs for the `--client-ca-file`
option - to authorize control plane components and end-users - and the `--requestheader-client-ca-file` option - to authorize aggregation apiserver requests.
#### Warning:
Do **not** reuse a CA that is used in a different context unless you understand
the risks and the mechanisms to protect the CA's usage.
If you are not running kube-proxy on a host running the API server,
then you must make sure that the system is enabled with the following
`kube-apiserver` flag:
```
`--enable-aggregator-routing=true
`
```