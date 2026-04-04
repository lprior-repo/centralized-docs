---
doc_id: ref/docs-concepts-overview-working-with-objects-annotations.md/docs-concepts-overview-working-with-objects-annotations
chunk_id: ref/docs-concepts-overview-working-with-objects-annotations.md/docs-concepts-overview-working-with-objects-annotations#2-standard
chunk_level: standard
chunk_type: prose
heading: What's next
token_count: 363
summary: ## Syntax and character set *Annotations* are key/value pairs. Valid annotation keys have two segments: an optional prefix and name, separated by a slash (`/`). The name segment is required and must...
---

## Syntax and character set
*Annotations* are key/value pairs. Valid annotation keys have two segments: an optional prefix and name, separated by a slash (`/`). The name segment is required and must be 63 characters or less, beginning and ending with an alphanumeric character (`[a-z0-9A-Z]`) with dashes (`-`), underscores (`\_`), dots (`.`), and alphanumerics between. The prefix is optional. If specified, the prefix must be a DNS subdomain: a series of DNS labels separated by dots (`.`), not longer than 253 characters in total, followed by a slash (`/`).
If the prefix is omitted, the annotation Key is presumed to be private to the user. Automated system components (e.g. `kube-scheduler`, `kube-controller-manager`, `kube-apiserver`, `kubectl`, or other third-party automation) which add annotations to end-user objects must specify a prefix.
The `kubernetes.io/` and `k8s.io/` prefixes are reserved for Kubernetes core components.
For example, here's a manifest for a Pod that has the annotation `imageregistry: https://hub.docker.com/` :
```
`apiVersion: v1
kind: Pod
metadata:
name: annotations-demo
annotations:
imageregistry: "https://hub.docker.com/"
spec:
containers:
- name: nginx
image: nginx:1.14.2
ports:
- containerPort: 80
`
```
## What's next
* Learn more about [Labels and Selectors](/docs/concepts/overview/working-with-objects/labels/).
* Find [Well-known labels, Annotations and Taints](/docs/reference/labels-annotations-taints/)