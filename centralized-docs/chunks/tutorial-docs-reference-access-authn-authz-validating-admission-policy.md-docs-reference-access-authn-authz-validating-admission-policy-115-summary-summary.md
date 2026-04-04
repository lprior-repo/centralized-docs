---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#115-summary
chunk_level: summary
chunk_type: prose
heading: Getting Started with Validating Admission Policy
token_count: 84
summary: ``` `kubectl create deploy --image=dev.example.com/nginx invalid ` ``` The error message is similar to this. ``` `error: failed to create deployment: deployments.apps \"invalid\" is forbidden:...
---

```
`kubectl create deploy --image=dev.example.com/nginx invalid
`
```
The error message is similar to this.
```
`error: failed to create deployment: deployments.apps "invalid" is forbidden: ValidatingAdmissionPolicy 'image-matches-namespace-environment.policy.example.com' with binding 'demo-binding-test.example.com' denied request: only prod images are allowed in namespace default
`
```