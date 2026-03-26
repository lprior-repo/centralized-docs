---
doc_id: ref/docs-reference-kubectl-generated-kubectlcluster-info-kubectlcluster-infodump.md/docs-reference-kubectl-generated-kubectlcluster-info-kubectlcluster-infodump
chunk_id: ref/docs-reference-kubectl-generated-kubectlcluster-info-kubectlcluster-infodump.md/docs-reference-kubectl-generated-kubectlcluster-info-kubectlcluster-infodump#8-summary
chunk_level: summary
chunk_type: table
heading: Options
token_count: 113
summary: | |--namespaces strings| || A comma separated list of namespaces to dump. | |-o, --output stringDefault: \"json\"| || Output format. One of: (json, yaml, kyaml, name, go-template, go-template-file,...
---

|
|--namespaces strings|
||
A comma separated list of namespaces to dump.
|
|-o, --output stringDefault: "json"|
||
Output format. One of: (json, yaml, kyaml, name, go-template, go-template-file, template, templatefile, jsonpath, jsonpath-as-json, jsonpath-file).
|
|--output-directory string|
||
Where to output the files. If empty or '-' uses stdout, otherwise creates a directory hierarchy in that directory
|
|--pod-running-timeout durationDefault: 20s|
||