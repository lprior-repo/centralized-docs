---
doc_id: ref/docs-reference-command-line-tools-reference-kube-scheduler.md/docs-reference-command-line-tools-reference-kube-scheduler
chunk_id: ref/docs-reference-command-line-tools-reference-kube-scheduler.md/docs-reference-command-line-tools-reference-kube-scheduler#12-summary
chunk_level: summary
chunk_type: table
heading: Options
token_count: 117
summary: If the component is not specified, defaults to \"kube\" | |--feature-gates colonSeparatedMultimapStringString| || Comma-separated list of component:key=value pairs that describe feature gates for...
---

If the component is not specified, defaults to "kube"
|
|--feature-gates colonSeparatedMultimapStringString|
||
Comma-separated list of component:key=value pairs that describe feature gates for alpha/experimental features of different components.
If the component is not specified, defaults to "kube". This flag can be repeatedly invoked. For example: --feature-gates 'wardle:featureA=true,wardle:featureB=false' --feature-gates 'kube:featureC=true'Options are:
kube:APIResponseCompression=true|false (BETA - default=true)