---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#30-summary
chunk_level: summary
chunk_type: prose
heading: API objects
token_count: 120
summary: * namespaced resources (like Pods), across all namespaces For example: you can use a ClusterRole to allow a particular user to run `kubectl get pods --all-namespaces` Here is an example of a...
---

* namespaced resources (like Pods), across all namespaces
For example: you can use a ClusterRole to allow a particular user to run
`kubectl get pods --all-namespaces`
Here is an example of a ClusterRole that can be used to grant read access to
[secrets](/docs/concepts/configuration/secret/) in any particular namespace,
or across all namespaces (depending on how it is [bound](#rolebinding-and-clusterrolebinding)):
[`access/simple-clusterrole.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/access/simple-clusterrole.yaml)