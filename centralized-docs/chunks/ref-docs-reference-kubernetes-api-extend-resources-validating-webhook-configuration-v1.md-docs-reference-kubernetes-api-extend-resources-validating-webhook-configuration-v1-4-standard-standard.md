---
doc_id: ref/docs-reference-kubernetes-api-extend-resources-validating-webhook-configuration-v1.md/docs-reference-kubernetes-api-extend-resources-validating-webhook-configuration-v1
chunk_id: ref/docs-reference-kubernetes-api-extend-resources-validating-webhook-configuration-v1.md/docs-reference-kubernetes-api-extend-resources-validating-webhook-configuration-v1#4-standard
chunk_level: standard
chunk_type: prose
heading: ValidatingWebhookConfiguration
token_count: 474
summary: * **webhooks.clientConfig.service.name** (string), required `name` is the name of the service. Required * **webhooks.clientConfig.service.namespace** (string), required `namespace` is the namespace...
---

* **webhooks.clientConfig.service.name** (string), required
`name` is the name of the service. Required
* **webhooks.clientConfig.service.namespace** (string), required
`namespace` is the namespace of the service. Required
* **webhooks.clientConfig.service.path** (string)
`path` is an optional URL path which will be sent in any request to this service.
* **webhooks.clientConfig.service.port** (int32)
If specified, the port on the service that hosting webhook. Default to 443 for backward compatibility. `port` should be a valid port number (1-65535, inclusive).
* **webhooks.clientConfig.url** (string)
`url` gives the location of the webhook, in standard URL form (`scheme://host:port/path`). Exactly one of `url` or `service` must be specified.
The `host` should not refer to a service running in the cluster; use the `service` field instead. The host might be resolved via external DNS in some apiservers (e.g., `kube-apiserver` cannot resolve in-cluster DNS as that would be a layering violation). `host` may also be an IP address.
Please note that using `localhost` or `127.0.0.1` as a `host` is risky unless you take great care to run this webhook on all hosts which run an apiserver which might need to make calls to this webhook. Such installs are likely to be non-portable, i.e., not easy to turn up in a new cluster.
The scheme must be "https"; the URL must begin with "https://".
A path is optional, and if present may be any string permissible in a URL. You may use the path to pass an arbitrary string to the webhook, for example, a cluster identifier.
Attempting to use a user or basic auth e.g. "user:password@" is not allowed. Fragments ("#...") and query parameters ("?...") are not allowed, either.
* **webhooks.name** (string), required
The name of the admission webhook. Name should be fully qualified, e.g., imagepolicy.kubernetes.io, where "imagepolicy" is the name of the webhook, and kubernetes.io is the name of the organization. Required.