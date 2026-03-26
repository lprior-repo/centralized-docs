---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#38-standard
chunk_level: standard
chunk_type: prose
heading: Related Pages
token_count: 449
summary: ### Permissive RBAC permissions You can replicate a permissive ABAC policy using RBAC role bindings. #### Warning: The following policy allows **ALL** service accounts to act as cluster...
---

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
Last modified January 16, 2026 at 12:49 AM PST: [Clarified RBAC doc about resourceNames field and create verb (#50455) (a14451f9ad)](https://github.com/kubernetes/website/commit/a14451f9ad5cf2b3117321114d00c1fb23c3b0b7)
## Related Pages

- [EndpointSlices](docs-concepts-services-networking-endpoint-slices.md)
- [Secrets](docs-concepts-configuration-secret.md)
- [Owners and Dependents](docs-concepts-overview-working-with-objects-owners-dependents.md)
- [Process ID Limits And Reservations](docs-concepts-policy-pid-limiting.md)
- [Configure the Aggregation Layer](docs-tasks-extend-kubernetes-configure-aggregation-layer.md)