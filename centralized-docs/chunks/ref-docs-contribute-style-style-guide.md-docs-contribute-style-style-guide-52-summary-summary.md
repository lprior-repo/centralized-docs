---
doc_id: ref/docs-contribute-style-style-guide.md/docs-contribute-style-style-guide
chunk_id: ref/docs-contribute-style-style-guide.md/docs-contribute-style-style-guide#52-summary
chunk_level: summary
chunk_type: prose
heading: Referring to Kubernetes API resources
token_count: 124
summary: Code examples and configuration examples that include version information should be consistent with the accompanying text. If the information is version specific, the Kubernetes version needs to be...
---

Code examples and configuration examples that include version information should
be consistent with the accompanying text.
If the information is version specific, the Kubernetes version needs to be defined
in the `prerequisites` section of the [Task template](/docs/contribute/style/page-content-types/#task)
or the [Tutorial template](/docs/contribute/style/page-content-types/#tutorial).
Once the page is saved, the `prerequisites` section is shown as **Before you begin**.
To specify the Kubernetes version for a task or tutorial page, include
`min-kubernetes-server-version` in the front matter of the page.