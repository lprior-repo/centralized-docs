---
doc_id: ref/docs-concepts-overview-working-with-objects-annotations.md/docs-concepts-overview-working-with-objects-annotations
chunk_id: ref/docs-concepts-overview-working-with-objects-annotations.md/docs-concepts-overview-working-with-objects-annotations#11-summary
chunk_level: summary
chunk_type: prose
heading: Syntax and character set
token_count: 106
summary: and `k8s.io/` prefixes are reserved for Kubernetes core components. For example, here's a manifest for a Pod that has the annotation `imageregistry: https://hub.docker.com/` : ``` `apiVersion: v1...
---

 and `k8s.io/` prefixes are reserved for Kubernetes core components.
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