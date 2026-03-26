---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#22-standard
chunk_level: standard
chunk_type: table
heading: `WebhookMatchCondition`
token_count: 304
summary: ## `WebhookMatchCondition` **Appears in:** * [WebhookConfiguration](#apiserver-k8s-io-v1beta1-WebhookConfiguration)|Field|Description| |`expression`**[Required]** `string`| expression represents the...
---

## `WebhookMatchCondition`
**Appears in:**
* [WebhookConfiguration](#apiserver-k8s-io-v1beta1-WebhookConfiguration)|Field|Description|
|`expression`**[Required]**
`string`|
expression represents the expression which will be evaluated by CEL. Must evaluate to bool.
CEL expressions have access to the contents of the SubjectAccessReview in v1 version.
If version specified by subjectAccessReviewVersion in the request variable is v1beta1,
the contents would be converted to the v1 version before evaluating the CEL expression.
* 'resourceAttributes' describes information for a resource access request and is unset for non-resource requests. e.g. has(request.resourceAttributes) &amp;&amp; request.resourceAttributes.namespace == 'default'
* 'nonResourceAttributes' describes information for a non-resource access request and is unset for resource requests. e.g. has(request.nonResourceAttributes) &amp;&amp; request.nonResourceAttributes.path == '/healthz'.
* 'user' is the user to test for. e.g. request.user == 'alice'
* 'groups' is the groups to test for. e.g. ('group1' in request.groups)
* 'extra' corresponds to the user.Info.GetExtra() method from the authenticator.
* 'uid' is the information about the requesting user. e.g. request.uid == '1'
Documentation on CEL: https://kubernetes.io/docs/reference/using-api/cel/
|