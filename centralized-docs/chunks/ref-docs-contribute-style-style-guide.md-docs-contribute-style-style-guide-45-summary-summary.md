---
doc_id: ref/docs-contribute-style-style-guide.md/docs-contribute-style-style-guide
chunk_id: ref/docs-contribute-style-style-guide.md/docs-contribute-style-style-guide#45-summary
chunk_level: summary
chunk_type: prose
heading: Referring to Kubernetes API resources
token_count: 124
summary: * *Object*: a resource that serves as a \"record of intent\". An object is a desired state for a specific part of your cluster, which the Kubernetes control plane tries to maintain. All objects in the...
---

* *Object*: a resource that serves as a "record of intent". An object is a desired
state for a specific part of your cluster, which the Kubernetes control plane tries to maintain.
All objects in the Kubernetes API are also resources.
For clarity, you can add "resource" or "object" when referring to an API resource in Kubernetes
documentation.
An example: write "a Secret object" instead of "a Secret".
If it is clear just from the capitalization, you don't need to add the extra word.
Consider rephrasing when that change helps avoid misunderstandings. A common situation is