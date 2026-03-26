---
doc_id: ref/docs-reference-kubectl-kubectl.md/docs-reference-kubectl-kubectl
chunk_id: ref/docs-reference-kubectl-kubectl.md/docs-reference-kubectl-kubectl#15-summary
chunk_level: summary
chunk_type: table
heading: Environment variables
token_count: 128
summary: |KUBECTL\_ENABLE\_CMD\_SHADOW| ||When set to true, external plugins can be used as subcommands for builtin commands if subcommand does not exist. In alpha stage, this feature can only be used for...
---

|KUBECTL\_ENABLE\_CMD\_SHADOW|
||When set to true, external plugins can be used as subcommands for builtin commands if subcommand does not exist. In alpha stage, this feature can only be used for create command(e.g. kubectl create networkpolicy).|
|KUBECTL\_PORT\_FORWARD\_WEBSOCKETS|
||When set to true, the kubectl port-forward command will attempt to stream using the websockets protocol. If the upgrade to websockets fails, the commands will fallback to use the current SPDY protocol.|
|KUBECTL\_REMOTE\_COMMAND