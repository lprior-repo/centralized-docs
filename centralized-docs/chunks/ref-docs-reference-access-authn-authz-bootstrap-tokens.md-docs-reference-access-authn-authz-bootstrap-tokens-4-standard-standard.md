---
doc_id: ref/docs-reference-access-authn-authz-bootstrap-tokens.md/docs-reference-access-authn-authz-bootstrap-tokens
chunk_id: ref/docs-reference-access-authn-authz-bootstrap-tokens.md/docs-reference-access-authn-authz-bootstrap-tokens#4-standard
chunk_level: standard
chunk_type: prose
heading: Related Pages
token_count: 389
summary: #### Warning: Any party with a bootstrapping token can create a valid signature for that token. When using ConfigMap signing it's discouraged to share the same token with many clients, since a...
---

#### Warning:
Any party with a bootstrapping token can create a valid signature for that
token. When using ConfigMap signing it's discouraged to share the same token with
many clients, since a compromised client can potentially man-in-the middle another
client relying on the signature to bootstrap TLS trust.
Consult the [kubeadm implementation details](/docs/reference/setup-tools/kubeadm/implementation-details/)
section for more information.
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
Last modified September 11, 2024 at 11:29 AM PST: [add hyperlink for RFC3339 in bootstrap-tokens.md (2e7c1d4935)](https://github.com/kubernetes/website/commit/2e7c1d4935e26d202a7a137677e28264a08d6c44)
## Related Pages

- [Implementation details](docs-reference-setup-tools-kubeadm-implementation-details.md)
- [Using RBAC Authorization](docs-reference-access-authn-authz-rbac.md)
- [Kubeadm](docs-reference-setup-tools-kubeadm.md)
- [Communication between Nodes and the Control Plane](docs-concepts-architecture-control-plane-node-communication.md)
- [Binding](docs-reference-kubernetes-api-workload-resources-binding-v1.md)