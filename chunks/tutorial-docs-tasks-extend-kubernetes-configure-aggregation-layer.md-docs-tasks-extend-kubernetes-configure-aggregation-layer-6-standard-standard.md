---
doc_id: tutorial/docs-tasks-extend-kubernetes-configure-aggregation-layer.md/docs-tasks-extend-kubernetes-configure-aggregation-layer
chunk_id: tutorial/docs-tasks-extend-kubernetes-configure-aggregation-layer.md/docs-tasks-extend-kubernetes-configure-aggregation-layer#6-standard
chunk_level: standard
chunk_type: prose
heading: Enable Kubernetes Apiserver flags
token_count: 135
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