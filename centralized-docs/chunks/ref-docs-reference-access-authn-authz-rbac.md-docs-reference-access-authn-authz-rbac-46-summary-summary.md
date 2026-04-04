---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#46-summary
chunk_level: summary
chunk_type: prose
heading: API objects
token_count: 91
summary: #### ClusterRoleBinding example To grant permissions across a whole cluster, you can use a ClusterRoleBinding. The following ClusterRoleBinding allows any user in the group \"manager\" to read secrets...
---

#### ClusterRoleBinding example
To grant permissions across a whole cluster, you can use a ClusterRoleBinding.
The following ClusterRoleBinding allows any user in the group "manager" to read
secrets in any namespace.
[`access/simple-clusterrolebinding.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/access/simple-clusterrolebinding.yaml)![](/images/copycode.svg "Copy access/simple-clusterrolebinding.yaml to clipboard")