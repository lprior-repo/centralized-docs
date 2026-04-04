---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#17-detailed
chunk_level: detailed
chunk_type: prose
heading: Upgrading from ABAC
token_count: 975
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
## Upgrading from ABAC
Clusters that originally ran older Kubernetes versions often used
permissive ABAC policies, including granting full API access to all
service accounts.
Default RBAC policies grant scoped permissions to control-plane components, nodes,
and controllers, but grant *no permissions* to service accounts outside the `kube-system` namespace
(beyond the permissions given by [API discovery roles](#discovery-roles)).
While far more secure, this can be disruptive to existing workloads expecting to automatically receive API permissions.
Here are two approaches for managing this transition:
### Parallel authorizers
Run both the RBAC and ABAC authorizers, and specify a policy file that contains
the [legacy ABAC policy](/docs/reference/access-authn-authz/abac/#policy-file-format):
```
`--authorization-mode=...,RBAC,ABAC --authorization-policy-file=mypolicy.json
`
```
To explain that first command line option in detail: if earlier authorizers, such as Node,
deny a request, then the RBAC authorizer attempts to authorize the API request. If RBAC
also denies that API request, the ABAC authorizer is then run. This means that any request
allowed by *either* the RBAC or ABAC policies is allowed.
When the kube-apiserver is run with a log level of 5 or higher for the RBAC component
(`--vmodule=rbac\*=5` or `--v=5`), you can see RBAC denials in the API server log
(prefixed with `RBAC`).
You can use that information to determine which roles need to be granted to which users, groups, or service accounts.
Once you have [granted roles to service accounts](#service-account-permissions) and workloads
are running with no RBAC denial messages in the server logs, you can remove the ABAC authorizer.
### Permissive RBAC permissions
You can replicate a permissive ABAC policy using RBAC role bindings.
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