---
doc_id: ref/docs-contribute-style-style-guide.md/docs-contribute-style-style-guide
chunk_id: ref/docs-contribute-style-style-guide.md/docs-contribute-style-style-guide#6-detailed
chunk_level: detailed
chunk_type: table
heading: Kubernetes.io word list
token_count: 742
summary: ### Don't include the command prompt Do and Don't - Don't include the command prompt|Do|Don't| |`kubectl get pods`|`$ kubectl get pods`| ### Separate commands from output Verify that the pod is...
---

### Don't include the command prompt
Do and Don't - Don't include the command prompt|Do|Don't|
|`kubectl get pods`|`$ kubectl get pods`|
### Separate commands from output
Verify that the pod is running on your chosen node:
```
`kubectl get pods --output=wide
`
```
The output is similar to this:
```
`NAME READY STATUS RESTARTS AGE IP NODE
nginx 1/1 Running 0 13s 10.200.0.4 worker0
`
```
### Versioning Kubernetes examples
Code examples and configuration examples that include version information should
be consistent with the accompanying text.
If the information is version specific, the Kubernetes version needs to be defined
in the `prerequisites` section of the [Task template](/docs/contribute/style/page-content-types/#task)
or the [Tutorial template](/docs/contribute/style/page-content-types/#tutorial).
Once the page is saved, the `prerequisites` section is shown as **Before you begin**.
To specify the Kubernetes version for a task or tutorial page, include
`min-kubernetes-server-version` in the front matter of the page.
If the example YAML is in a standalone file, find and review the topics that include it as a reference.
Verify that any topics using the standalone YAML have the appropriate version information defined.
If a stand-alone YAML file is not referenced from any topics, consider deleting it instead of updating it.
For example, if you are writing a tutorial that is relevant to Kubernetes version 1.8,
the front-matter of your markdown file should look something like:
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
## Formulae and equations
You can use the Docsy support for [diagrams and formulae](https://www.docsy.dev/docs/adding-content/diagrams-and-formulae/#latex-support-with-katex).
For example: `\\\\(\\frac{7}{9} \\sqrt{K^8 s}\\\\)`, which renders as \\(\\frac{7}{9} \\sqrt{K^8 s}\\).
Prefer inline formulae where reasonable, but you can use a `math` block if that's likely to help readers.
Read the Docsy guide to find out what you need to change in your page to activate support;
if you have problems, add `math: true` to the page [front matter](https://gohugo.io/content-management/front-matter/)
(you can do this even if you think the automatic activation should be enough).
## Kubernetes.io word list
A list of Kubernetes-specific terms and words to be used consistently across the site.
Kubernetes.io word list|Term|Usage|
|Kubernetes|Kubernetes should always be capitalized.|
|Docker|Docker should always be capitalized.|
|SIG Docs|SIG Docs rather than SIG-DOCS or other variations.|
|On-premises|On-premises or On-prem rather than On-premise or other variations.|
|cloud native|Cloud native or cloud native as appropriate for sentence structure rather than cloud-native or Cloud Native.|
|open source|Open source or open source as appropriate for sentence structure rather than open-source or Open Source.|