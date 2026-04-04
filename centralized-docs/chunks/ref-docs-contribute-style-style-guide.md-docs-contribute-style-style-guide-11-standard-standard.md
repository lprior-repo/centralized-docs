---
doc_id: ref/docs-contribute-style-style-guide.md/docs-contribute-style-style-guide
chunk_id: ref/docs-contribute-style-style-guide.md/docs-contribute-style-style-guide#11-standard
chunk_level: standard
chunk_type: table
heading: Referring to Kubernetes API resources
token_count: 464
summary: ### When to use Kubernetes API terminologies The different Kubernetes API terminologies are: * *API kinds*: the name used in the API URL (such as `pods`, `namespaces`). API kinds are sometimes also...
---

### When to use Kubernetes API terminologies
The different Kubernetes API terminologies are:
* *API kinds*: the name used in the API URL (such as `pods`, `namespaces`).
API kinds are sometimes also called *resource types*.
* *API resource*: a single instance of an API kind (such as `pod`, `secret`).
* *Object*: a resource that serves as a "record of intent". An object is a desired
state for a specific part of your cluster, which the Kubernetes control plane tries to maintain.
All objects in the Kubernetes API are also resources.
For clarity, you can add "resource" or "object" when referring to an API resource in Kubernetes
documentation.
An example: write "a Secret object" instead of "a Secret".
If it is clear just from the capitalization, you don't need to add the extra word.
Consider rephrasing when that change helps avoid misunderstandings. A common situation is
when you want to start a sentence with an API kind, such as “Secret”; because English
and other languages capitalize at the start of sentences, readers cannot tell whether you
mean the API kind or the general concept. Rewording can help.
### API resource names
Always format API resource names using [UpperCamelCase](https://en.wikipedia.org/wiki/Camel_case),
also known as PascalCase. Do not write API kinds with code formatting.
Don't split an API object name into separate words. For example, use PodTemplateList, not Pod Template List.
For more information about PascalCase and code formatting, review the related guidance on
[Use upper camel case for API objects](/docs/contribute/style/style-guide/#use-upper-camel-case-for-api-objects)
and [Use code style for inline code, commands, and API objects](/docs/contribute/style/style-guide/#code-style-inline-code).
For more information about Kubernetes API terminologies, review the related
guidance on [Kubernetes API terminology](/docs/reference/using-api/api-concepts/#standard-api-terminology).
### Don't include the command prompt
Do and Don't - Don't include the command prompt|Do|Don't|
|`kubectl get pods`|`$ kubectl get pods`|