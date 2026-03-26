---
doc_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1
chunk_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1#13-detailed
chunk_level: detailed
chunk_type: prose
heading: ValidatingAdmissionPolicyBinding
token_count: 1021
summary: * **spec.paramRef.name** (string) name is the name of the resource being referenced. One of `name` or `selector` must be set, but `name` and `selector` are mutually exclusive properties. If one is...
---

* **spec.paramRef.name** (string)
name is the name of the resource being referenced.
One of `name` or `selector` must be set, but `name` and `selector` are mutually exclusive properties. If one is set, the other must be unset.
A single parameter used for all admission requests can be configured by setting the `name` field, leaving `selector` blank, and setting namespace if `paramKind` is namespace-scoped.
* **spec.paramRef.namespace** (string)
namespace is the namespace of the referenced resource. Allows limiting the search for params to a specific namespace. Applies to both `name` and `selector` fields.
A per-namespace parameter may be used by specifying a namespace-scoped `paramKind` in the policy and leaving this field empty.
* If `paramKind` is cluster-scoped, this field MUST be unset. Setting this field results in a configuration error.
* If `paramKind` is namespace-scoped, the namespace of the object being evaluated for admission will be used when this field is left unset. Take care that if this is left empty the binding must not match any cluster-scoped resources, which will result in an error.
* **spec.paramRef.parameterNotFoundAction** (string)
`parameterNotFoundAction` controls the behavior of the binding when the resource exists, and name or selector is valid, but there are no parameters matched by the binding. If the value is set to `Allow`, then no matched parameters will be treated as successful validation by the binding. If set to `Deny`, then no matched parameters will be subject to the `failurePolicy` of the policy.
Allowed values are `Allow` or `Deny`
Required
* **spec.paramRef.selector** ([LabelSelector](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/label-selector/#LabelSelector))
selector can be used to match multiple param objects based on their labels. Supply selector: {} to match all resources of the ParamKind.
If multiple params are found, they are all evaluated with the policy expressions and the results are ANDed together.
One of `name` or `selector` must be set, but `name` and `selector` are mutually exclusive properties. If one is set, the other must be unset.
* **spec.policyName** (string)
PolicyName references a ValidatingAdmissionPolicy name which the ValidatingAdmissionPolicyBinding binds to. If the referenced resource does not exist, this binding is considered invalid and will be ignored Required.
* **spec.validationActions** ([]string)
*Set: unique values will be kept during a merge*
validationActions declares how Validations of the referenced ValidatingAdmissionPolicy are enforced. If a validation evaluates to false it is always enforced according to these actions.
Failures defined by the ValidatingAdmissionPolicy's FailurePolicy are enforced according to these actions only if the FailurePolicy is set to Fail, otherwise the failures are ignored. This includes compilation errors, runtime errors and misconfigurations of the policy.
validationActions is declared as a set of action values. Order does not matter. validationActions may not contain duplicates of the same action.
The supported actions values are:
"Deny" specifies that a validation failure results in a denied request.
"Warn" specifies that a validation failure is reported to the request client in HTTP Warning headers, with a warning code of 299. Warnings can be sent both for allowed or denied admission responses.
"Audit" specifies that a validation failure is included in the published audit event for the request. The audit event will contain a `validation.policy.admission.k8s.io/validation\_failure` audit annotation with a value containing the details of the validation failures, formatted as a JSON list of objects, each with the following fields: - message: The validation failure message string - policy: The resource name of the ValidatingAdmissionPolicy - binding: The resource name of the ValidatingAdmissionPolicyBinding - expressionIndex: The index of the failed validations in the ValidatingAdmissionPolicy - validationActions: The enforcement actions enacted for the validation failure Example audit annotation: `"validation.policy.admission.k8s.io/validation\_failure": "[{\\"message\\": \\"Invalid value\\", {\\"policy\\": \\"policy.example.com\\", {\\"binding\\": \\"policybinding.example.com\\", {\\"expressionIndex\\": \\"1\\", {\\"validationActions\\": [\\"Audit\\"]}]"`
Clients should expect to handle additional values by ignoring any values not recognized.
"Deny" and "Warn" may not be used together since this combination needlessly duplicates the validation failure both in the API response body and the HTTP warning headers.
Required.
#### Parameters
* **name** (*in path*): string, required
name of the ValidatingAdmissionPolicy
* **pretty** (*in query*): string
[pretty](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#pretty)