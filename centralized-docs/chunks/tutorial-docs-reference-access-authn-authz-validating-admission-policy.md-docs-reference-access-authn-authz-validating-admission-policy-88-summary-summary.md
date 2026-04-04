---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#88-summary
chunk_level: summary
chunk_type: prose
heading: Getting Started with Validating Admission Policy
token_count: 103
summary: To return a more friendly message when the policy rejects a request, we can use a CEL expression to composite a message with `spec.validations[i].messageExpression`. Similar to the validation...
---

To return a more friendly message when the policy rejects a request, we can use a CEL expression
to composite a message with `spec.validations[i].messageExpression`. Similar to the validation expression,
a message expression has access to `object`, `oldObject`, `request`, `params`, and `namespaceObject`.
Unlike validations, message expression must evaluate to a string.
For example, to better inform the user of the reason of denial when the policy refers to a parameter,
we can have the following validation: