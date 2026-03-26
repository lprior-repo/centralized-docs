---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#14-standard
chunk_level: standard
chunk_type: prose
heading: API objects
token_count: 443
summary: #### Note: You cannot restrict **deletecollection** or top-level **create** requests by resource name. For **create**, this limitation is because the name of the new object may not be known at...
---

#### Note:
You cannot restrict **deletecollection** or top-level **create** requests by resource name.
For **create**, this limitation is because the name of the new object may not be known at authorization time. However, the **create** limitation applies only to top-level resources, not subresources. For example, you can use the `resourceNames` field with `pods/exec`.
If you restrict **list** or **watch** by `resourceName`, clients must include a `metadata.name` field selector in their **list** or **watch** request (that matches the specified `resourceName`)
in order to be authorized.
For example: `kubectl get configmaps --field-selector=metadata.name=my-configmap`
Rather than referring to individual `resources`, `apiGroups`, and `verbs`,
you can use the wildcard `\*` symbol to refer to all such objects.
For `nonResourceURLs`, you can use the wildcard `\*` as a suffix glob match.
For `resourceNames`, an empty set means that everything is allowed.
Here is an example that allows access to perform any current and future action on
all current and future resources in the `example.com` API group.
This is similar to the built-in `cluster-admin` role.
```
`apiVersion: rbac.authorization.k8s.io/v1
kind: Role
metadata:
namespace: default
name: example.com-superuser # DO NOT USE THIS ROLE, IT IS JUST AN EXAMPLE
rules:
- apiGroups: ["example.com"]
resources: ["\*"]
verbs: ["\*"]
`
```
#### Caution:
Using wildcards in resource and verb entries could result in overly permissive access being granted
to sensitive resources.
For instance, if a new resource type is added, or a new subresource is added,
or a new custom verb is checked, the wildcard entry automatically grants access, which may be undesirable.
The [principle of least privilege](/docs/concepts/security/rbac-good-practices/#least-privilege)
should be employed, using specific resources and verbs to ensure only the permissions required for the
workload to function correctly are applied.