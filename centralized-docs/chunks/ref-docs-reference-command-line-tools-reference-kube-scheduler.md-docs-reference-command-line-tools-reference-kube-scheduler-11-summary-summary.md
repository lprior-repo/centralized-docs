---
doc_id: ref/docs-reference-command-line-tools-reference-kube-scheduler.md/docs-reference-command-line-tools-reference-kube-scheduler
chunk_id: ref/docs-reference-command-line-tools-reference-kube-scheduler.md/docs-reference-command-line-tools-reference-kube-scheduler#11-summary
chunk_level: summary
chunk_type: table
heading: Options
token_count: 116
summary: | |--emulated-version strings| || The versions different components emulate their capabilities (APIs, features, ...) of. If set, the component will emulate the behavior of this version instead of the...
---

|
|--emulated-version strings|
||
The versions different components emulate their capabilities (APIs, features, ...) of.
If set, the component will emulate the behavior of this version instead of the underlying binary version.
Version format could only be major.minor, for example: '--emulated-version=wardle=1.2,kube=1.31'.
Options are: kube=1.32..1.35(default:1.35)
If the component is not specified, defaults to "kube"
|
|--feature-gates colonSeparatedMultimapStringString|
||