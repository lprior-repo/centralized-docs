---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#1-detailed
chunk_level: detailed
chunk_type: prose
heading: Table of Contents
token_count: 971
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
- [This cluster role binding allows anyone in the "manager" group to read secrets in any namespace.](#this-cluster-role-binding-allows-anyone-in-the-manager-group-to-read-secrets-in-any-namespace)
    - [Referring to resources](#referring-to-resources)
- [at the HTTP level, the name of the resource for accessing ConfigMap](#at-the-http-level-the-name-of-the-resource-for-accessing-configmap)
- [objects is "configmaps"](#objects-is-configmaps)
      - [Note:](#note)
      - [Caution:](#caution)
    - [Aggregated ClusterRoles](#aggregated-clusterroles)
      - [Caution:](#caution)
- [the rules below will be added to the "monitoring" ClusterRole.](#the-rules-below-will-be-added-to-the-monitoring-clusterrole)
- [Add these permissions to the "admin" and "edit" default roles.](#add-these-permissions-to-the-admin-and-edit-default-roles)
- [Add these permissions to the "view" default role.](#add-these-permissions-to-the-view-default-role)
      - [Role examples](#role-examples)
- [at the HTTP level, the name of the resource for accessing Pod](#at-the-http-level-the-name-of-the-resource-for-accessing-pod)
- [objects is "pods"](#objects-is-pods)
- [at the HTTP level, the name of the resource for accessing Deployment](#at-the-http-level-the-name-of-the-resource-for-accessing-deployment)
- [objects is "deployments"](#objects-is-deployments)
- [at the HTTP level, the name of the resource for accessing Pod](#at-the-http-level-the-name-of-the-resource-for-accessing-pod)
- [objects is "pods"](#objects-is-pods)
- [at the HTTP level, the name of the resource for accessing Job](#at-the-http-level-the-name-of-the-resource-for-accessing-job)
- [objects is "jobs"](#objects-is-jobs)
- [at the HTTP level, the name of the resource for accessing ConfigMap](#at-the-http-level-the-name-of-the-resource-for-accessing-configmap)
- [objects is "configmaps"](#objects-is-configmaps)
- [at the HTTP level, the name of the resource for accessing Node](#at-the-http-level-the-name-of-the-resource-for-accessing-node)