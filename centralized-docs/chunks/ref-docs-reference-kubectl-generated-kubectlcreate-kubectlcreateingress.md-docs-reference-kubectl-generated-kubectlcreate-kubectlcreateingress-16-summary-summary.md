---
doc_id: ref/docs-reference-kubectl-generated-kubectlcreate-kubectlcreateingress.md/docs-reference-kubectl-generated-kubectlcreate-kubectlcreateingress
chunk_id: ref/docs-reference-kubectl-generated-kubectlcreate-kubectlcreateingress.md/docs-reference-kubectl-generated-kubectlcreate-kubectlcreateingress#16-summary
chunk_level: summary
chunk_type: table
heading: Options
token_count: 128
summary: ]. Paths containing the leading character '\*' are considered pathType=Prefix. tls argument is optional. | |--save-config| || If true, the configuration of current object will be saved in its...
---

]. Paths containing the leading character '\*' are considered pathType=Prefix. tls argument is optional.
|
|--save-config|
||
If true, the configuration of current object will be saved in its annotation. Otherwise, the annotation will be unchanged. This flag is useful when you want to perform kubectl apply on this object in the future.
|
|--show-managed-fields|
||
If true, keep the managedFields when printing objects in JSON or YAML format.
|
|--template string|
||
Template string or path to template file to use when -o=go-template, -o=go-template-file. The template format is golang templates