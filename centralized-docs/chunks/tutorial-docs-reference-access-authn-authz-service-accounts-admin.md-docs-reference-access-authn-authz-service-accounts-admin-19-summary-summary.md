---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#19-summary
chunk_level: summary
chunk_type: prose
heading: User accounts versus service accounts
token_count: 106
summary: ### Verifying and inspecting private claims The TokenReview API can be used to verify and extract private claims from a token: 1. First, assume you have a pod named `test-pod` and a service account...
---

### Verifying and inspecting private claims
The TokenReview API can be used to verify and extract private claims from a token:
1. First, assume you have a pod named `test-pod` and a service account named `my-sa`.
2. Create a token that is bound to this Pod:
```
`kubectl create token my-sa --bound-object-kind="Pod" --bound-object-name="test-pod"
`
```
3. Copy this token into a new file named `tokenreview.yaml`: