---
doc_id: ref/docs-reference-command-line-tools-reference-kube-scheduler.md/docs-reference-command-line-tools-reference-kube-scheduler
chunk_id: ref/docs-reference-command-line-tools-reference-kube-scheduler.md/docs-reference-command-line-tools-reference-kube-scheduler#44-summary
chunk_level: summary
chunk_type: table
heading: Options
token_count: 107
summary: | |--min-compatibility-version strings| || The min version of control plane components the server should be compatible with. Must be less or equal to the emulated-version. Version format could only...
---

|
|--min-compatibility-version strings|
||
The min version of control plane components the server should be compatible with.
Must be less or equal to the emulated-version. Version format could only be major.minor, for example: '--min-compatibility-version=wardle=1.2,kube=1.31'.
Options are: kube=1.32..1.35(default:1.34)
If the component is not specified, defaults to "kube"
|
|--permit-address-sharing|
||
If true, SO\