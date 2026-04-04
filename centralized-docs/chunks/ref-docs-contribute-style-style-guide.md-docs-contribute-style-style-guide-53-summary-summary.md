---
doc_id: ref/docs-contribute-style-style-guide.md/docs-contribute-style-style-guide
chunk_id: ref/docs-contribute-style-style-guide.md/docs-contribute-style-style-guide#53-summary
chunk_level: summary
chunk_type: prose
heading: Referring to Kubernetes API resources
token_count: 121
summary: . To specify the Kubernetes version for a task or tutorial page, include `min-kubernetes-server-version` in the front matter of the page. If the example YAML is in a standalone file, find and review...
---

.
To specify the Kubernetes version for a task or tutorial page, include
`min-kubernetes-server-version` in the front matter of the page.
If the example YAML is in a standalone file, find and review the topics that include it as a reference.
Verify that any topics using the standalone YAML have the appropriate version information defined.
If a stand-alone YAML file is not referenced from any topics, consider deleting it instead of updating it.
For example, if you are writing a tutorial that is relevant to Kubernetes version 1.8,
the front-matter of your markdown file should look something like: