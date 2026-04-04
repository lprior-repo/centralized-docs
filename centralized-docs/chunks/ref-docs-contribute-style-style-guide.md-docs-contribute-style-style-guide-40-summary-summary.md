---
doc_id: ref/docs-contribute-style-style-guide.md/docs-contribute-style-style-guide
chunk_id: ref/docs-contribute-style-style-guide.md/docs-contribute-style-style-guide#40-summary
chunk_level: summary
chunk_type: prose
heading: Referring to Kubernetes API resources
token_count: 126
summary: Kubernetes uses the word *resource* to refer to API resources. For example, the URL path `/apis/apps/v1/namespaces/default/deployments/my-app` represents a Deployment named \"my-app\" in the \"default\"...
---

Kubernetes uses the word *resource* to refer to API resources. For example,
the URL path `/apis/apps/v1/namespaces/default/deployments/my-app` represents a
Deployment named "my-app" in the "default"
[namespace](/docs/concepts/overview/working-with-objects/namespaces). In HTTP jargon,
[namespace](/docs/concepts/overview/working-with-objects/namespaces) is a resource -
the same way that all web URLs identify a resource.
Kubernetes documentation also uses "resource" to talk about CPU and memory
requests and limits. It'