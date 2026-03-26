---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#36-standard
chunk_level: standard
chunk_type: prose
heading: Write access for EndpointSlices
token_count: 438
summary: ## Write access for EndpointSlices Kubernetes clusters created before Kubernetes v1.22 include write access to EndpointSlices (and the now-deprecated Endpoints API) in the aggregated \"edit\" and...
---

## Write access for EndpointSlices
Kubernetes clusters created before Kubernetes v1.22 include write access to
EndpointSlices (and the now-deprecated Endpoints API) in the aggregated "edit" and "admin" roles.
As a mitigation for [CVE-2021-25740](https://github.com/kubernetes/kubernetes/issues/103675),
this access is not part of the aggregated roles in clusters that you create using
Kubernetes v1.22 or later.
Existing clusters that have been upgraded to Kubernetes v1.22 will not be
subject to this change. The [CVE
announcement](https://github.com/kubernetes/kubernetes/issues/103675) includes
guidance for restricting this access in existing clusters.
If you want new clusters to retain this level of access in the aggregated roles,
you can create the following ClusterRole:
[`access/endpoints-aggregated.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/access/endpoints-aggregated.yaml)![](/images/copycode.svg "Copy access/endpoints-aggregated.yaml to clipboard")
```
`apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRole
metadata:
annotations:
kubernetes.io/description: |-
Add endpoints write permissions to the edit and admin roles. This was
removed by default in 1.22 because of CVE-2021-25740. See
https://issue.k8s.io/103675. This can allow writers to direct LoadBalancer
or Ingress implementations to expose backend IPs that would not otherwise
be accessible, and can circumvent network policies or security controls
intended to prevent/isolate access to those backends.
EndpointSlices were never included in the edit or admin roles, so there
is nothing to restore for the EndpointSlice API.
labels:
rbac.authorization.k8s.io/aggregate-to-edit: "true"
name: custom:aggregate-to-edit:endpoints # you can change this if you wish
rules:
- apiGroups: [""]
resources: ["endpoints"]
verbs: ["create", "delete", "deletecollection", "patch", "update"]
`
```