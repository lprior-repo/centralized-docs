---
doc_id: ops/general/docs-concept-the-logic-of-cue
chunk_id: ops/general/docs-concept-the-logic-of-cue#24-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 135
summary: or the much simpler file-based approaches as used in HCL and Kustomize,. finding a declaration for a concrete field value does not guarantee
---

or the much simpler file-based approaches as used in HCL and Kustomize,
finding a declaration for a concrete field value does not guarantee
a final answer,
because another concrete value that occurs elsewhere can override it.
When one needs to change a value of such a field,
it can be time-consuming and,
especially when under pressure,
very tempting to skip following complicated inheritance chains,
double-check a configuration file specifying overlay order,
or look for a file that is lexically sorted after the one under consideration.
