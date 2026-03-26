---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#77-summary
chunk_level: summary
chunk_type: prose
heading: API objects
token_count: 46
summary: ``` `rules: - nonResourceURLs: [\"/healthz\", \"/healthz/\*\"] # '\*' in a nonResourceURL is a suffix glob match verbs: [\"get\", \"post\"] ` ```
---

```
`rules:
- nonResourceURLs: ["/healthz", "/healthz/\*"] # '\*' in a nonResourceURL is a suffix glob match
verbs: ["get", "post"]
`
```