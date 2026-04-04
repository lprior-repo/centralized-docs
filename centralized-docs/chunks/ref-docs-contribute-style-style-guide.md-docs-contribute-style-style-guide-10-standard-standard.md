---
doc_id: ref/docs-contribute-style-style-guide.md/docs-contribute-style-style-guide
chunk_id: ref/docs-contribute-style-style-guide.md/docs-contribute-style-style-guide#10-standard
chunk_level: standard
chunk_type: prose
heading: Referring to Kubernetes API resources
token_count: 267
summary: ## Referring to Kubernetes API resources This section talks about how we reference API resources in the documentation. ### Clarification about \"resource\" Kubernetes uses the word *resource* to refer...
---

## Referring to Kubernetes API resources
This section talks about how we reference API resources in the documentation.
### Clarification about "resource"
Kubernetes uses the word *resource* to refer to API resources. For example,
the URL path `/apis/apps/v1/namespaces/default/deployments/my-app` represents a
Deployment named "my-app" in the "default"
[namespace](/docs/concepts/overview/working-with-objects/namespaces). In HTTP jargon,
[namespace](/docs/concepts/overview/working-with-objects/namespaces) is a resource -
the same way that all web URLs identify a resource.
Kubernetes documentation also uses "resource" to talk about CPU and memory
requests and limits. It's very often a good idea to refer to API resources
as "API resources"; that helps to avoid confusion with CPU and memory resources,
or with other kinds of resource.
If you are using the lowercase plural form of a resource name, such as
`deployments` or `configmaps`, provide extra written context to help readers
understand what you mean. If you are using the term in a context where the
UpperCamelCase name could work too, and there is a risk of ambiguity,
consider using the API kind in UpperCamelCase.