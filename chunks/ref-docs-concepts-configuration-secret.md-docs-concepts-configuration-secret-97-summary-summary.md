---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#97-summary
chunk_level: summary
chunk_type: prose
heading: Types of Secret
token_count: 87
summary: #### Optional Secrets When you reference a Secret in a Pod, you can mark the Secret as *optional*, such as in the following example. If an optional Secret doesn't exist, Kubernetes ignores it....
---

#### Optional Secrets
When you reference a Secret in a Pod, you can mark the Secret as *optional*,
such as in the following example. If an optional Secret doesn't exist,
Kubernetes ignores it.
[`secret/optional-secret.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/secret/optional-secret.yaml)![](/images/copycode.svg "Copy secret/optional-secret.yaml to clipboard")