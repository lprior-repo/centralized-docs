---
doc_id: ref/docs-concepts-overview-working-with-objects-annotations.md/docs-concepts-overview-working-with-objects-annotations
chunk_id: ref/docs-concepts-overview-working-with-objects-annotations.md/docs-concepts-overview-working-with-objects-annotations#10-summary
chunk_level: summary
chunk_type: prose
heading: Syntax and character set
token_count: 118
summary: `.`), not longer than 253 characters in total, followed by a slash (`/`). If the prefix is omitted, the annotation Key is presumed to be private to the user. Automated system components (e.g....
---

`.`), not longer than 253 characters in total, followed by a slash (`/`).
If the prefix is omitted, the annotation Key is presumed to be private to the user. Automated system components (e.g. `kube-scheduler`, `kube-controller-manager`, `kube-apiserver`, `kubectl`, or other third-party automation) which add annotations to end-user objects must specify a prefix.
The `kubernetes.io/` and `k8s.io/` prefixes are reserved for Kubernetes core components.
For example, here's a manifest for a Pod that has the annotation