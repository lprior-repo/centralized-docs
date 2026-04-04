---
doc_id: ref/docs-contribute-style-style-guide.md/docs-contribute-style-style-guide
chunk_id: ref/docs-contribute-style-style-guide.md/docs-contribute-style-style-guide#4-detailed
chunk_level: detailed
chunk_type: table
heading: Language
token_count: 319
summary: ### Starting a sentence with a component tool or component name Do and Don't - Starting a sentence with a component tool or component name|Do|Don't| |The `kubeadm` tool bootstraps and provisions...
---

### Starting a sentence with a component tool or component name
Do and Don't - Starting a sentence with a component tool or component name|Do|Don't|
|The `kubeadm` tool bootstraps and provisions machines in a cluster.|`kubeadm` tool bootstraps and provisions machines in a cluster.|
|The kube-scheduler is the default scheduler for Kubernetes.|kube-scheduler is the default scheduler for Kubernetes.|
### Use a general descriptor over a component name
Do and Don't - Use a general descriptor over a component name|Do|Don't|
|The Kubernetes API server offers an OpenAPI spec.|The apiserver offers an OpenAPI spec.|
|Aggregated APIs are subordinate API servers.|Aggregated APIs are subordinate APIServers.|
### Use normal style for string and integer field values
For field values of type string or integer, use normal style without quotation marks.
Do and Don't - Use normal style for string and integer field values|Do|Don't|
|Set the value of `imagePullPolicy` to Always.|Set the value of `imagePullPolicy` to "Always".|
|Set the value of `image` to nginx:1.16.|Set the value of `image` to `nginx:1.16`.|
|Set the value of the `replicas` field to 2.|Set the value of the `replicas` field to `2`.|
However, consider quoting values where there is a risk that readers might confuse the value
with an API kind.