---
doc_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation
chunk_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation#7-detailed
chunk_level: detailed
chunk_type: prose
heading: Related Pages
token_count: 568
summary: ## Auditing An audit event is logged for each impersonation request to help track how impersonation is used. When a request uses constrained impersonation, the audit event includes...
---

## Auditing
An audit event is logged for each impersonation request to help track how impersonation is used.
When a request uses constrained impersonation, the audit event includes `authenticationMetadata`
object with an `impersonationConstraint` field that indicates which constrained impersonation verb
was used to authorize the request.
Example audit event:
```
`{
"kind": "Event",
"apiVersion": "audit.k8s.io/v1",
"user": {
"username": "system:serviceaccount:default:my-controller"
},
"impersonatedUser": {
"username": "jane.doe@example.com"
},
"authenticationMetadata": {
"impersonationConstraint": "impersonate:user-info"
},
"verb": "list",
"objectRef": {
"resource": "pods",
"namespace": "default"
}
}
`
```
The `impersonationConstraint` value indicates which mode was used (for example, `impersonate:user-info`,
`impersonate:associated-node`). The specific action (for example, `list`) can be determined from the
`verb` field in the audit event.
## What's next
* Read about [RBAC authorization](/docs/reference/access-authn-authz/rbac/)
* Understand [Kubernetes authentication](/docs/reference/access-authn-authz/authentication/)
## Feedback
Was this page helpful?
Yes
No
Thanks for the feedback. If you have a specific, answerable question about how to use Kubernetes, ask it on
[Stack Overflow](https://stackoverflow.com/questions/tagged/kubernetes).
Open an issue in the [GitHub Repository](https://www.github.com/kubernetes/website/) if you want to
[report a problem](<https://github.com/kubernetes/website/issues/new?title=Issue with k8s.io>)
or
[suggest an improvement](<https://github.com/kubernetes/website/issues/new?title=Improvement for k8s.io>).
Last modified November 21, 2025 at 4:01 PM PST: [Resolve review comments (aa6ec11c3b)](https://github.com/kubernetes/website/commit/aa6ec11c3b45296a56be89c56e3b26df628c5a89)
## Related Pages

- [Configure the Aggregation Layer](docs-tasks-extend-kubernetes-configure-aggregation-layer.md)
- [Creating a cluster with kubeadm](docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md)
- [Service Accounts](docs-concepts-security-service-accounts.md)
- [Secrets](docs-concepts-configuration-secret.md)
- [Using RBAC Authorization](docs-reference-access-authn-authz-rbac.md)