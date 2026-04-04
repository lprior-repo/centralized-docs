---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#92-summary
chunk_level: summary
chunk_type: prose
heading: Getting Started with Validating Admission Policy
token_count: 65
summary: messageExpression: \"'object.spec.replicas must be no greater than ' + string(params.maxReplicas)\" reason: Invalid ` ``` After creating a params object that limits the replicas to 3 and setting up the...
---

messageExpression: "'object.spec.replicas must be no greater than ' + string(params.maxReplicas)"
reason: Invalid
`
```
After creating a params object that limits the replicas to 3 and setting up the binding,
when we try to create a deployment with 5 replicas, we will receive the following message.