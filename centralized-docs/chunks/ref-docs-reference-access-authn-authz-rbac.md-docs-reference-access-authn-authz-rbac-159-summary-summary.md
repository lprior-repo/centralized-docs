---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#159-summary
chunk_level: summary
chunk_type: prose
heading: Upgrading from ABAC
token_count: 123
summary: #### Warning: The following policy allows **ALL** service accounts to act as cluster administrators. Any application running in a container receives service account credentials automatically, and...
---

#### Warning:
The following policy allows **ALL** service accounts to act as cluster administrators.
Any application running in a container receives service account credentials automatically,
and could perform any action against the API, including viewing secrets and modifying permissions.
This is not a recommended policy.
```
`kubectl create clusterrolebinding permissive-binding \\
--clusterrole=cluster-admin \\
--user=admin \\
--user=kubelet \\
--group=system:serviceaccounts
`
```
After you have transitioned to use RBAC, you should adjust the access controls
for your cluster to ensure that these meet your information security needs.