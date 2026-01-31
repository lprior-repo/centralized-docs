---
doc_id: ops/general/docs-concept-how-cue-works-with-go
chunk_id: ops/general/docs-concept-how-cue-works-with-go#7-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 131
summary: using CUE can be found in CUE By Example, in. Controlling Kubernetes with CUE [https://github
---

using CUE can be found in CUE By Example, in
Controlling Kubernetes with CUE [https://github.com/cue-labs/cue-by-example/blob/main/003_kubernetes_tutorial/README.md].

The example above relies on generating CUE within the cue.mod/gen directory
of the CUE module that holds a configuration,
but we are working on a system for providing schemas for well-known services at
a well-known location.
This will remove the need to generate such CUE locally –
see discussion #2939 [/issue/2939] for more details.

USING CUE’S GO API
