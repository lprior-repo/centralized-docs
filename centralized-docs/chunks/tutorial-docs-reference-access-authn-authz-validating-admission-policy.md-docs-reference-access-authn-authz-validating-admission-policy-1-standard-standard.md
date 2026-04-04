---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#1-standard
chunk_level: standard
chunk_type: prose
heading: What Resources Make a Policy
token_count: 359
summary: # Validating Admission Policy FEATURE STATE: `Kubernetes v1.30 [stable]` This page provides an overview of Validating Admission Policy. ## What is Validating Admission Policy? Validating admission...
---

# Validating Admission Policy
FEATURE STATE:
`Kubernetes v1.30 [stable]`
This page provides an overview of Validating Admission Policy.
## What is Validating Admission Policy?
Validating admission policies offer a declarative, in-process alternative to validating admission webhooks.
Validating admission policies use the Common Expression Language (CEL) to declare the validation
rules of a policy.
Validation admission policies are highly configurable, enabling policy authors to define policies
that can be parameterized and scoped to resources as needed by cluster administrators.
## What Resources Make a Policy
A policy is generally made up of three resources:
* The `ValidatingAdmissionPolicy` describes the abstract logic of a policy
(think: "this policy makes sure a particular label is set to a particular value").
* A parameter resource provides information to a ValidatingAdmissionPolicy to make it a concrete
statement (think "the `owner` label must be set to something that ends in `.company.com`").
A native type such as ConfigMap or a CRD defines the schema of a parameter resource.
`ValidatingAdmissionPolicy` objects specify what Kind they are expecting for their parameter resource.
* A `ValidatingAdmissionPolicyBinding` links the above resources together and provides scoping.
If you only want to require an `owner` label to be set for `Pods`, the binding is where you would
specify this restriction.
At least a `ValidatingAdmissionPolicy` and a corresponding `ValidatingAdmissionPolicyBinding`
must be defined for a policy to have an effect.
If a `ValidatingAdmissionPolicy` does not need to be configured via parameters, simply leave
`spec.paramKind` in `ValidatingAdmissionPolicy` not specified.