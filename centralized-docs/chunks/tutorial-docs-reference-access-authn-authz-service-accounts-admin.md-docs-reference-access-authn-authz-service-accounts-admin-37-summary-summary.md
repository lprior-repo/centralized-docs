---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#37-summary
chunk_level: summary
chunk_type: prose
heading: Bound service account token volume mechanism
token_count: 126
summary: ``` `... - name: kube-api-access-&lt;random-suffix&gt; projected: sources: - serviceAccountToken: path: token # must match the path the app expects - configMap: items: - key: ca.crt path: ca.crt...
---

```
`...
- name: kube-api-access-&lt;random-suffix&gt;
projected:
sources:
- serviceAccountToken:
path: token # must match the path the app expects
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
That manifest snippet defines a projected volume that consists of three sources. In this case,
each source also represents a single path within that volume. The three sources are: