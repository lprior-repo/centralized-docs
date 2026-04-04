---
doc_id: ref/docs-contribute-style-style-guide.md/docs-contribute-style-style-guide
chunk_id: ref/docs-contribute-style-style-guide.md/docs-contribute-style-style-guide#5-standard
chunk_level: standard
chunk_type: table
heading: Language
token_count: 498
summary: ## Language Kubernetes documentation has been translated into multiple languages (see [Localization READMEs](https://github.com/kubernetes/website/blob/main/README.md#localization-readmemds)). The...
---

## Language
Kubernetes documentation has been translated into multiple languages
(see [Localization READMEs](https://github.com/kubernetes/website/blob/main/README.md#localization-readmemds)).
The way of localizing the docs for a different language is described in [Localizing Kubernetes Documentation](/docs/contribute/localization/).
The English-language documentation uses U.S. English spelling and grammar.
### Use upper camel case for API objects
When you refer specifically to interacting with an API object, use
[UpperCamelCase](https://en.wikipedia.org/wiki/Camel_case), also known as
Pascal case. You may see different capitalization, such as "configMap",
in the [API Reference](/docs/reference/kubernetes-api/). When writing
general documentation, it's better to use upper camel case, calling it "ConfigMap" instead.
When you are generally discussing an API object, use
[sentence-style capitalization](https://docs.microsoft.com/en-us/style-guide/text-formatting/using-type/use-sentence-style-capitalization).
The following examples focus on capitalization. For more information about formatting
API object names, review the related guidance on [Code Style](#code-style-inline-code).
Do and Don't - Use Pascal case for API objects|Do|Don't|
|The HorizontalPodAutoscaler resource is responsible for ...|The Horizontal pod autoscaler is responsible for ...|
|A PodList object is a list of pods.|A Pod List object is a list of pods.|
|The Volume object contains a `hostPath` field.|The volume object contains a hostPath field.|
|Every ConfigMap object is part of a namespace.|Every configMap object is part of a namespace.|
|For managing confidential data, consider using the Secret API.|For managing confidential data, consider using the secret API.|
### Use angle brackets for placeholders
Use angle brackets for placeholders. Tell the reader what a placeholder
represents, for example:
Display information about a pod:
```
`kubectl describe pod &lt;pod-name&gt; -n &lt;namespace&gt;
`
```
If the namespace of the pod is `default`, you can omit the '-n' parameter.
### Use bold for user interface elements
Do and Don't - Bold interface elements|Do|Don't|
|Click **Fork**.|Click "Fork".|
|Select **Other**.|Select "Other".|