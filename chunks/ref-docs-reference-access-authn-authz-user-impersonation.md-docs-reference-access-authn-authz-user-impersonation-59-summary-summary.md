---
doc_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation
chunk_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation#59-summary
chunk_level: summary
chunk_type: prose
heading: Constrained Impersonation
token_count: 99
summary: The controller would get the node name using the downward API: ``` `env: - name: MY\_NODE\_NAME valueFrom: fieldRef: fieldPath: spec.nodeName ` ``` Then configure the kubeconfig to impersonate: ```...
---

The controller would get the node name using the downward API:
```
`env:
- name: MY\_NODE\_NAME
valueFrom:
fieldRef:
fieldPath: spec.nodeName
`
```
Then configure the kubeconfig to impersonate:
```
`kubeConfig, \_ := clientcmd.BuildConfigFromFlags("", "")
kubeConfig.Impersonate = rest.ImpersonationConfig{
UserName: "system:node:" + os.Getenv("MY\_NODE\_NAME"),
}
`
```