---
doc_id: ref/docs-tasks-administer-cluster-certificates.md/docs-tasks-administer-cluster-certificates
chunk_id: ref/docs-tasks-administer-cluster-certificates.md/docs-tasks-administer-cluster-certificates#6-standard
chunk_level: standard
chunk_type: prose
heading: Related Pages
token_count: 494
summary: ## Distributing Self-Signed CA Certificate A client node may refuse to recognize a self-signed CA certificate as valid. For a non-production deployment, or for a deployment that runs behind a company...
---

## Distributing Self-Signed CA Certificate
A client node may refuse to recognize a self-signed CA certificate as valid.
For a non-production deployment, or for a deployment that runs behind a company
firewall, you can distribute a self-signed CA certificate to all clients and
refresh the local list for valid certificates.
On each client, perform the following operations:
```
`sudo cp ca.crt /usr/local/share/ca-certificates/kubernetes.crt
sudo update-ca-certificates
`
```
```
`Updating certificates in /etc/ssl/certs...
1 added, 0 removed; done.
Running hooks in /etc/ca-certificates/update.d....
done.
`
```
## Certificates API
You can use the `certificates.k8s.io` API to provision
x509 certificates to use for authentication as documented
in the [Managing TLS in a cluster](/docs/tasks/tls/managing-tls-in-a-cluster/)
task page.
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
Last modified February 03, 2026 at 9:28 PM PST: [Correct openssl command for certificate generation (7d3c779543)](https://github.com/kubernetes/website/commit/7d3c77954336cc7882d8dd9967a68c3d98b3491a)
## Related Pages

- [Certificates and Certificate Signing Requests](docs-reference-access-authn-authz-certificate-signing-requests.md)
- [Communication between Nodes and the Control Plane](docs-concepts-architecture-control-plane-node-communication.md)
- [Binding](docs-reference-kubernetes-api-workload-resources-binding-v1.md)
- [conventions](docs-reference-kubectl-conventions.md)
- [HorizontalPodAutoscaler](docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md)