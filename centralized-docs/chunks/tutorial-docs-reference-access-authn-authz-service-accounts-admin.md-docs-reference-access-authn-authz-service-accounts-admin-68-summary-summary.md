---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#68-summary
chunk_level: summary
chunk_type: prose
heading: Auto-generated legacy ServiceAccount token clean up
token_count: 121
summary: ``` `... - name: kube-api-access-&lt;random-suffix&gt; projected: defaultMode: 420 # decimal equivalent of octal 0644 sources: - serviceAccountToken: expirationSeconds: 3607 path: token - configMap:...
---

```
`...
- name: kube-api-access-&lt;random-suffix&gt;
projected:
defaultMode: 420 # decimal equivalent of octal 0644
sources:
- serviceAccountToken:
expirationSeconds: 3607
path: token
- configMap:
items:
- key: ca.crt
path: ca.crt
name: kube-root-ca.crt
- downwardAPI:
items:
- fieldRef:
apiVersion: v1
fieldPath: metadata.namespace
path: namespace
`
```
That manifest snippet defines a projected volume that combines information from three sources: