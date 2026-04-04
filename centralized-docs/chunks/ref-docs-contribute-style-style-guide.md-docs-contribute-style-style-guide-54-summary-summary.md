---
doc_id: ref/docs-contribute-style-style-guide.md/docs-contribute-style-style-guide
chunk_id: ref/docs-contribute-style-style-guide.md/docs-contribute-style-style-guide#54-summary
chunk_level: summary
chunk_type: prose
heading: Referring to Kubernetes API resources
token_count: 80
summary: ``` `--- title: &lt;your tutorial title here&gt; min-kubernetes-server-version: v1.8 --- ` ``` In code and configuration examples, do not include comments about alternative versions. Be careful to...
---

```
`---
title: &lt;your tutorial title here&gt;
min-kubernetes-server-version: v1.8
---
`
```
In code and configuration examples, do not include comments about alternative versions.
Be careful to not include incorrect statements in your examples as comments, such as:
```
`apiVersion: v1 # earlier versions use...
kind: Pod
...
`
```