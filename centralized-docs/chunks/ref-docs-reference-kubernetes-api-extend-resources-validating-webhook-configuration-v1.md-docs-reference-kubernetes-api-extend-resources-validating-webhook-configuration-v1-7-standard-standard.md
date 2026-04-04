---
doc_id: ref/docs-reference-kubernetes-api-extend-resources-validating-webhook-configuration-v1.md/docs-reference-kubernetes-api-extend-resources-validating-webhook-configuration-v1
chunk_id: ref/docs-reference-kubernetes-api-extend-resources-validating-webhook-configuration-v1.md/docs-reference-kubernetes-api-extend-resources-validating-webhook-configuration-v1#7-standard
chunk_level: standard
chunk_type: prose
heading: ValidatingWebhookConfiguration
token_count: 468
summary: * If failurePolicy=Fail, reject the request * If failurePolicy=Ignore, the error is ignored and the webhook is skipped *MatchCondition represents a condition which must by fulfilled for a request to...
---

* If failurePolicy=Fail, reject the request
* If failurePolicy=Ignore, the error is ignored and the webhook is skipped
*MatchCondition represents a condition which must by fulfilled for a request to be sent to a webhook.*
* **webhooks.matchConditions.expression** (string), required
Expression represents the expression which will be evaluated by CEL. Must evaluate to bool. CEL expressions have access to the contents of the AdmissionRequest and Authorizer, organized into CEL variables:
'object' - The object from the incoming request. The value is null for DELETE requests. 'oldObject' - The existing object. The value is null for CREATE requests. 'request' - Attributes of the admission request(/pkg/apis/admission/types.go#AdmissionRequest). 'authorizer' - A CEL Authorizer. May be used to perform authorization checks for the principal (user or service account) of the request.
See [https://pkg.go.dev/k8s.io/apiserver/pkg/cel/library#Authz](https://pkg.go.dev/k8s.io/apiserver/pkg/cel/library#Authz)
'authorizer.requestResource' - A CEL ResourceCheck constructed from the 'authorizer' and configured with the
request resource.
Documentation on CEL: [https://kubernetes.io/docs/reference/using-api/cel/](https://kubernetes.io/docs/reference/using-api/cel/)
Required.
* **webhooks.matchConditions.name** (string), required
Name is an identifier for this match condition, used for strategic merging of MatchConditions, as well as providing an identifier for logging purposes. A good name should be descriptive of the associated expression. Name must be a qualified name consisting of alphanumeric characters, '-', '*' or '.', and must start and end with an alphanumeric character (e.g. 'MyName', or 'my.name', or '123-abc', regex used for validation is '([A-Za-z0-9][-A-Za-z0-9*.]\*)?[A-Za-z0-9]') with an optional DNS subdomain prefix and '/' (e.g. 'example.com/MyName')
Required.
* **webhooks.matchPolicy** (string)
matchPolicy defines how the "rules" list is used to match incoming requests. Allowed values are "Exact" or "Equivalent".