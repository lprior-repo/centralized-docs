---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#1-standard
chunk_level: standard
chunk_type: prose
heading: Table of Contents
token_count: 455
summary: - [Using RBAC Authorization](#using-rbac-authorization)   - [API objects](#api-objects)       - [Caution:](#caution)     - [Role and ClusterRole](#role-and-clusterrole)       - [Role...
---

- [Using RBAC Authorization](#using-rbac-authorization)
  - [API objects](#api-objects)
      - [Caution:](#caution)
    - [Role and ClusterRole](#role-and-clusterrole)
      - [Role example](#role-example)
      - [ClusterRole example](#clusterrole-example)
- ["namespace" omitted since ClusterRoles are not namespaced](#namespace-omitted-since-clusterroles-are-not-namespaced)
- [at the HTTP level, the name of the resource for accessing Secret](#at-the-http-level-the-name-of-the-resource-for-accessing-secret)
- [objects is "secrets"](#objects-is-secrets)
    - [RoleBinding and ClusterRoleBinding](#rolebinding-and-clusterrolebinding)
      - [RoleBinding examples](#rolebinding-examples)
- [This role binding allows "jane" to read pods in the "default" namespace.](#this-role-binding-allows-jane-to-read-pods-in-the-default-namespace)
- [You need to already have a Role named "pod-reader" in that namespace.](#you-need-to-already-have-a-role-named-pod-reader-in-that-namespace)
- [You can specify more than one "subject"](#you-can-specify-more-than-one-subject)
- ["roleRef" specifies the binding to a Role / ClusterRole](#roleref-specifies-the-binding-to-a-role--clusterrole)
- [This role binding allows "dave" to read secrets in the "development" namespace.](#this-role-binding-allows-dave-to-read-secrets-in-the-development-namespace)
- [You need to already have a ClusterRole named "secret-reader".](#you-need-to-already-have-a-clusterrole-named-secret-reader)
- [The namespace of the RoleBinding determines where the permissions are granted.](#the-namespace-of-the-rolebinding-determines-where-the-permissions-are-granted)
- [This only grants permissions within the "development" namespace.](#this-only-grants-permissions-within-the-development-namespace)
      - [ClusterRoleBinding example](#clusterrolebinding-example)