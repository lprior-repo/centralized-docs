---
doc_id: tutorial/docs-tasks-administer-cluster-dns-debugging-resolution.md/docs-tasks-administer-cluster-dns-debugging-resolution
chunk_id: tutorial/docs-tasks-administer-cluster-dns-debugging-resolution.md/docs-tasks-administer-cluster-dns-debugging-resolution#11-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 124
summary: #### Note: This example creates a pod in the `default` namespace. DNS name resolution for services depends on the namespace of the pod. For more information, review [DNS for Services and...
---

#### Note:
This example creates a pod in the `default` namespace. DNS name resolution for
services depends on the namespace of the pod. For more information, review
[DNS for Services and Pods](/docs/concepts/services-networking/dns-pod-service/#what-things-get-dns-names).
Use that manifest to create a Pod:
```
`kubectl apply -f https://k8s.io/examples/admin/dns/dnsutils.yaml
`
```
```
`pod/dnsutils created
`
```
…and verify its status:
```
`kubectl get pods dnsutils
`
```