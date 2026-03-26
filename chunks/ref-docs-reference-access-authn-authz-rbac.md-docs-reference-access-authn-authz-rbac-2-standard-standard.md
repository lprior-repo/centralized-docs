---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#2-standard
chunk_level: standard
chunk_type: prose
heading: Table of Contents
token_count: 504
summary: - [The namespace of the RoleBinding determines where the permissions are granted.](#the-namespace-of-the-rolebinding-determines-where-the-permissions-are-granted) - [This only grants permissions...
---

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