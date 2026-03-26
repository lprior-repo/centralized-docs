---
doc_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1
chunk_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1#3-detailed
chunk_level: detailed
chunk_type: prose
heading: ValidatingAdmissionPolicy
token_count: 999
summary: * If failurePolicy=Fail, reject the request * If failurePolicy=Ignore, the policy is skipped *MatchCondition represents a condition which must by fulfilled for a request to be sent to a webhook.* *...
---

* If failurePolicy=Fail, reject the request
* If failurePolicy=Ignore, the policy is skipped
*MatchCondition represents a condition which must by fulfilled for a request to be sent to a webhook.*
* **spec.matchConditions.expression** (string), required
Expression represents the expression which will be evaluated by CEL. Must evaluate to bool. CEL expressions have access to the contents of the AdmissionRequest and Authorizer, organized into CEL variables:
'object' - The object from the incoming request. The value is null for DELETE requests. 'oldObject' - The existing object. The value is null for CREATE requests. 'request' - Attributes of the admission request(/pkg/apis/admission/types.go#AdmissionRequest). 'authorizer' - A CEL Authorizer. May be used to perform authorization checks for the principal (user or service account) of the request.
See [https://pkg.go.dev/k8s.io/apiserver/pkg/cel/library#Authz](https://pkg.go.dev/k8s.io/apiserver/pkg/cel/library#Authz)
'authorizer.requestResource' - A CEL ResourceCheck constructed from the 'authorizer' and configured with the
request resource.
Documentation on CEL: [https://kubernetes.io/docs/reference/using-api/cel/](https://kubernetes.io/docs/reference/using-api/cel/)
Required.
* **spec.matchConditions.name** (string), required
Name is an identifier for this match condition, used for strategic merging of MatchConditions, as well as providing an identifier for logging purposes. A good name should be descriptive of the associated expression. Name must be a qualified name consisting of alphanumeric characters, '-', '*' or '.', and must start and end with an alphanumeric character (e.g. 'MyName', or 'my.name', or '123-abc', regex used for validation is '([A-Za-z0-9][-A-Za-z0-9*.]\*)?[A-Za-z0-9]') with an optional DNS subdomain prefix and '/' (e.g. 'example.com/MyName')
Required.
* **spec.matchConstraints** (MatchResources)
MatchConstraints specifies what resources this policy is designed to validate. The AdmissionPolicy cares about a request if it matches *all* Constraints. However, in order to prevent clusters from being put into an unstable state that cannot be recovered from via the API ValidatingAdmissionPolicy cannot match ValidatingAdmissionPolicy and ValidatingAdmissionPolicyBinding. Required.
*MatchResources decides whether to run the admission control policy on an object based on whether it meets the match criteria. The exclude rules take precedence over include rules (if a resource matches both, it is excluded)*
* **spec.matchConstraints.excludeResourceRules** ([]NamedRuleWithOperations)
*Atomic: will be replaced during a merge*
ExcludeResourceRules describes what operations on what resources/subresources the ValidatingAdmissionPolicy should not care about. The exclude rules take precedence over include rules (if a resource matches both, it is excluded)
*NamedRuleWithOperations is a tuple of Operations and Resources with ResourceNames.*
* **spec.matchConstraints.excludeResourceRules.apiGroups** ([]string)
*Atomic: will be replaced during a merge*
APIGroups is the API groups the resources belong to. '*' is all groups. If '*' is present, the length of the slice must be one. Required.
* **spec.matchConstraints.excludeResourceRules.apiVersions** ([]string)
*Atomic: will be replaced during a merge*
APIVersions is the API versions the resources belong to. '*' is all versions. If '*' is present, the length of the slice must be one. Required.
* **spec.matchConstraints.excludeResourceRules.operations** ([]string)
*Atomic: will be replaced during a merge*
Operations is the operations the admission hook cares about - CREATE, UPDATE, DELETE, CONNECT or \* for all of those operations and any future admission operations that are added. If '\*' is present, the length of the slice must be one. Required.
* **spec.matchConstraints.excludeResourceRules.resourceNames** ([]string)
*Atomic: will be replaced during a merge*
ResourceNames is an optional white list of names that the rule applies to. An empty set means that everything is allowed.
* **spec.matchConstraints.excludeResourceRules.resources** ([]string)
*Atomic: will be replaced during a merge*
Resources is a list of resources this rule applies to.
For example: 'pods' means pods. 'pods/log' means the log subresource of pods. '*' means all resources, but not subresources. 'pods/*' means all subresources of pods. '*/scale' means all scale subresources. '*/\*' means all resources and their subresources.
If wildcard is present, the validation rule will ensure resources do not overlap with each other.
Depending on the enclosing object, subresources might not be allowed. Required.