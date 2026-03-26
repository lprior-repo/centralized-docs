---
doc_id: tutorial/docs-tasks-extend-kubernetes-configure-aggregation-layer.md/docs-tasks-extend-kubernetes-configure-aggregation-layer
chunk_id: tutorial/docs-tasks-extend-kubernetes-configure-aggregation-layer.md/docs-tasks-extend-kubernetes-configure-aggregation-layer#3-standard
chunk_level: standard
chunk_type: prose
heading: Authentication Flow
token_count: 360
summary: ### Kubernetes Apiserver Proxies the Request The Kubernetes apiserver now will send, or proxy, the request to the extension apiserver that registered to handle the request. In order to do so, it...
---

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