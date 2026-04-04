---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#99-summary
chunk_level: summary
chunk_type: prose
heading: What's next
token_count: 100
summary: // The signature for the JWT. // Already wrapped in URL-safe base64, exactly as it appears in the final segment of the JWT. string signature = 2; } ` ``` ## Clean up If you created a namespace...
---

// The signature for the JWT.
// Already wrapped in URL-safe base64, exactly as it appears in the final segment of the JWT.
string signature = 2;
}
`
```
## Clean up
If you created a namespace `examplens` to experiment with, you can remove it:
```
`kubectl delete namespace examplens
`
```
## What's next
* Read more details about [projected volumes](/docs/concepts/storage/projected-volumes/).