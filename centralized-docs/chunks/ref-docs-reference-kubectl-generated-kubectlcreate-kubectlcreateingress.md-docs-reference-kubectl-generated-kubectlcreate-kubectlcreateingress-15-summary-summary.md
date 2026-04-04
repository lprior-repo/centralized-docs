---
doc_id: ref/docs-reference-kubectl-generated-kubectlcreate-kubectlcreateingress.md/docs-reference-kubectl-generated-kubectlcreate-kubectlcreateingress
chunk_id: ref/docs-reference-kubectl-generated-kubectlcreate-kubectlcreateingress.md/docs-reference-kubectl-generated-kubectlcreate-kubectlcreateingress#15-summary
chunk_level: summary
chunk_type: table
heading: Options
token_count: 107
summary: | |-h, --help| || help for ingress | |-o, --output string| || Output format. One of: (json, yaml, kyaml, name, go-template, go-template-file, template, templatefile, jsonpath, jsonpath-as-json,...
---

|
|-h, --help|
||
help for ingress
|
|-o, --output string|
||
Output format. One of: (json, yaml, kyaml, name, go-template, go-template-file, template, templatefile, jsonpath, jsonpath-as-json, jsonpath-file).
|
|--rule strings|
||
Rule in format host/path=service:port[,tls=secretname]. Paths containing the leading character '\*' are considered pathType=Prefix. tls argument is optional.
|
|--save-config|
||