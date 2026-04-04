---
doc_id: ref/docs-reference-kubernetes-api-extend-resources-validating-webhook-configuration-v1.md/docs-reference-kubernetes-api-extend-resources-validating-webhook-configuration-v1
chunk_id: ref/docs-reference-kubernetes-api-extend-resources-validating-webhook-configuration-v1.md/docs-reference-kubernetes-api-extend-resources-validating-webhook-configuration-v1#3-detailed
chunk_level: detailed
chunk_type: prose
heading: ValidatingWebhookConfiguration
token_count: 924
summary: * **apiVersion**: admissionregistration.k8s.io/v1 * **kind**: ValidatingWebhookConfiguration * **metadata**...
---

* **apiVersion**: admissionregistration.k8s.io/v1
* **kind**: ValidatingWebhookConfiguration
* **metadata** ([ObjectMeta](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/object-meta/#ObjectMeta))
Standard object metadata; More info: [https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata](https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata).
* **webhooks** ([]ValidatingWebhook)
*Patch strategy: merge on key `name`*
*Map: unique values on key name will be kept during a merge*
Webhooks is a list of webhooks and the affected resources and operations.
*ValidatingWebhook describes an admission webhook and the resources and operations it applies to.*
* **webhooks.admissionReviewVersions** ([]string), required
*Atomic: will be replaced during a merge*
AdmissionReviewVersions is an ordered list of preferred `AdmissionReview` versions the Webhook expects. API server will try to use first version in the list which it supports. If none of the versions specified in this list supported by API server, validation will fail for this object. If a persisted webhook configuration specifies allowed versions and does not include any versions known to the API Server, calls to the webhook will fail and be subject to the failure policy.
* **webhooks.clientConfig** (WebhookClientConfig), required
ClientConfig defines how to communicate with the hook. Required
*WebhookClientConfig contains the information to make a TLS connection with the webhook*
* **webhooks.clientConfig.caBundle** ([]byte)
`caBundle` is a PEM encoded CA bundle which will be used to validate the webhook's server certificate. If unspecified, system trust roots on the apiserver are used.
* **webhooks.clientConfig.service** (ServiceReference)
`service` is a reference to the service for this webhook. Either `service` or `url` must be specified.
If the webhook is running within the cluster, then you should use `service`.
*ServiceReference holds a reference to Service.legacy.k8s.io*
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