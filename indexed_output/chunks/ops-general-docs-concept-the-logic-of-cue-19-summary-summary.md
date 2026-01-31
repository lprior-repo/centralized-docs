---
doc_id: ops/general/docs-concept-the-logic-of-cue
chunk_id: ops/general/docs-concept-the-logic-of-cue#19-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 132
summary: (two references starting from the root of a config end up at the same node). and PF(π)=t means the type
---

(two references starting from the root of a config end up at the same node)
and PF(π)=t means the type
at path π is t (itself a graph in F).

Unification F⊓F′ of two TFSs F and F′ is then the greatest lower
bound of F and F′ in F ordered by subsumption.

This highly abstract definition determines almost everything about CUE.
For instance, lazy binding was not a design decision,
but a direct consequence of following this definition.
It determines the possible evaluation strategies and
what cycles mean, if allowed.
