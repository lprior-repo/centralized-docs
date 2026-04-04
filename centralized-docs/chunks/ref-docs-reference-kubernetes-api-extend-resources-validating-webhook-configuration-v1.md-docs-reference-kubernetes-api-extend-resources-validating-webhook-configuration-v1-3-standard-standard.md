---
doc_id: ref/docs-reference-kubernetes-api-extend-resources-validating-webhook-configuration-v1.md/docs-reference-kubernetes-api-extend-resources-validating-webhook-configuration-v1
chunk_id: ref/docs-reference-kubernetes-api-extend-resources-validating-webhook-configuration-v1.md/docs-reference-kubernetes-api-extend-resources-validating-webhook-configuration-v1#3-standard
chunk_level: standard
chunk_type: prose
heading: ValidatingWebhookConfiguration
token_count: 501
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