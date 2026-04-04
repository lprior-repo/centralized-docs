---
doc_id: ref/docs-contribute-style-style-guide.md/docs-contribute-style-style-guide
chunk_id: ref/docs-contribute-style-style-guide.md/docs-contribute-style-style-guide#41-summary
chunk_level: summary
chunk_type: prose
heading: Referring to Kubernetes API resources
token_count: 123
summary: Kubernetes documentation also uses \"resource\" to talk about CPU and memory requests and limits. It's very often a good idea to refer to API resources as \"API resources\"; that helps to avoid confusion...
---

Kubernetes documentation also uses "resource" to talk about CPU and memory
requests and limits. It's very often a good idea to refer to API resources
as "API resources"; that helps to avoid confusion with CPU and memory resources,
or with other kinds of resource.
If you are using the lowercase plural form of a resource name, such as
`deployments` or `configmaps`, provide extra written context to help readers
understand what you mean. If you are using the term in a context where the
UpperCamelCase name could work too, and there is a risk of ambiguity,