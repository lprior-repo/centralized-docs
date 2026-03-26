---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#59-summary
chunk_level: summary
chunk_type: prose
heading: API objects
token_count: 127
summary: field selector in their **list** or **watch** request (that matches the specified `resourceName`) in order to be authorized. For example: `kubectl get configmaps...
---

 field selector in their **list** or **watch** request (that matches the specified `resourceName`)
in order to be authorized.
For example: `kubectl get configmaps --field-selector=metadata.name=my-configmap`
Rather than referring to individual `resources`, `apiGroups`, and `verbs`,
you can use the wildcard `\*` symbol to refer to all such objects.
For `nonResourceURLs`, you can use the wildcard `\*` as a suffix glob match.
For `resourceNames`, an empty set means that everything is allowed.
Here is an example that allows access to perform any current and future action on