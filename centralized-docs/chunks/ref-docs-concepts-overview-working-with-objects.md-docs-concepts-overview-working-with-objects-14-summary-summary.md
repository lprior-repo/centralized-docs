---
doc_id: ref/docs-concepts-overview-working-with-objects.md/docs-concepts-overview-working-with-objects
chunk_id: ref/docs-concepts-overview-working-with-objects.md/docs-concepts-overview-working-with-objects#14-summary
chunk_level: summary
chunk_type: prose
heading: Understanding Kubernetes objects
token_count: 103
summary: One way to create a Deployment using a manifest file like the one above is to use the [`kubectl apply`](/docs/reference/generated/kubectl/kubectl-commands#apply) command in the `kubectl` command-line...
---

One way to create a Deployment using a manifest file like the one above is to use the
[`kubectl apply`](/docs/reference/generated/kubectl/kubectl-commands#apply) command
in the `kubectl` command-line interface, passing the `.yaml` file as an argument. Here's an example:
```
`kubectl apply -f https://k8s.io/examples/application/deployment.yaml
`
```
The output is similar to this:
```
`deployment.apps/nginx-deployment created
`
```