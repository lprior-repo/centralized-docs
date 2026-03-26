---
doc_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1
chunk_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1#2-detailed
chunk_level: detailed
chunk_type: prose
heading: ValidatingAdmissionPolicy
token_count: 919
summary: ## ValidatingAdmissionPolicy ValidatingAdmissionPolicy describes the definition of an admission validation policy that accepts or rejects an object without changing it. * **apiVersion**:...
---

## ValidatingAdmissionPolicy
ValidatingAdmissionPolicy describes the definition of an admission validation policy that accepts or rejects an object without changing it.
* **apiVersion**: admissionregistration.k8s.io/v1
* **kind**: ValidatingAdmissionPolicy
* **metadata** ([ObjectMeta](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/object-meta/#ObjectMeta))
Standard object metadata; More info: [https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata](https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata).
* **spec** (ValidatingAdmissionPolicySpec)
Specification of the desired behavior of the ValidatingAdmissionPolicy.
*ValidatingAdmissionPolicySpec is the specification of the desired behavior of the AdmissionPolicy.*
* **spec.auditAnnotations** ([]AuditAnnotation)
*Atomic: will be replaced during a merge*
auditAnnotations contains CEL expressions which are used to produce audit annotations for the audit event of the API request. validations and auditAnnotations may not both be empty; a least one of validations or auditAnnotations is required.
*AuditAnnotation describes how to produce an audit annotation for an API request.*
* **spec.auditAnnotations.key** (string), required
key specifies the audit annotation key. The audit annotation keys of a ValidatingAdmissionPolicy must be unique. The key must be a qualified name ([A-Za-z0-9][-A-Za-z0-9\_.]\*) no more than 63 bytes in length.
The key is combined with the resource name of the ValidatingAdmissionPolicy to construct an audit annotation key: "{ValidatingAdmissionPolicy name}/{key}".
If an admission webhook uses the same resource name as this ValidatingAdmissionPolicy and the same audit annotation key, the annotation key will be identical. In this case, the first annotation written with the key will be included in the audit event and all subsequent annotations with the same key will be discarded.
Required.
* **spec.auditAnnotations.valueExpression** (string), required
valueExpression represents the expression which is evaluated by CEL to produce an audit annotation value. The expression must evaluate to either a string or null value. If the expression evaluates to a string, the audit annotation is included with the string value. If the expression evaluates to null or empty string the audit annotation will be omitted. The valueExpression may be no longer than 5kb in length. If the result of the valueExpression is more than 10kb in length, it will be truncated to 10kb.
If multiple ValidatingAdmissionPolicyBinding resources match an API request, then the valueExpression will be evaluated for each binding. All unique values produced by the valueExpressions will be joined together in a comma-separated list.
Required.
* **spec.failurePolicy** (string)
failurePolicy defines how to handle failures for the admission policy. Failures can occur from CEL expression parse errors, type check errors, runtime errors and invalid or mis-configured policy definitions or bindings.
A policy is invalid if spec.paramKind refers to a non-existent Kind. A binding is invalid if spec.paramRef.name refers to a non-existent resource.
failurePolicy does not define how validations that evaluate to false are handled.
When failurePolicy is set to Fail, ValidatingAdmissionPolicyBinding validationActions define how failures are enforced.
Allowed values are Ignore or Fail. Defaults to Fail.
Possible enum values:
* `"Fail"` means that an error calling the webhook causes the admission to fail.
* `"Ignore"` means that an error calling the webhook is ignored.
* **spec.matchConditions** ([]MatchCondition)
*Patch strategy: merge on key `name`*
*Map: unique values on key name will be kept during a merge*
MatchConditions is a list of conditions that must be met for a request to be validated. Match conditions filter requests that have already been matched by the rules, namespaceSelector, and objectSelector. An empty list of matchConditions matches all requests. There are a maximum of 64 match conditions allowed.
If a parameter object is provided, it can be accessed via the `params` handle in the same manner as validation expressions.
The exact matching logic is (in order):
1. If ANY matchCondition evaluates to FALSE, the policy is skipped.
2. If ALL matchConditions evaluate to TRUE, the policy is evaluated.
3. If any matchCondition evaluates to an error (but none are FALSE):