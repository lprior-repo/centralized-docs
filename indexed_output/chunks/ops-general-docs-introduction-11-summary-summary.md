---
doc_id: ops/general/docs-introduction
chunk_id: ops/general/docs-introduction#11-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 139
summary: defined as a sequence of path-leaf values. For instance,
---

defined as a sequence of path-leaf values.
For instance,


Copy code
Copied!

{
    "a": 3,
    "b": {
        "c": "foo"
    }
}

could be represented as


Copy code
Copied!

"a": 3
"b": "c": "foo"

All the information of the original JSON file is retained in this
representation.

CUE generalizes this notion to the following pattern:


Copy code
Copied!

<set of nodes>: <constraints>

Each field declaration in CUE defines a set of nodes to which to apply
a specific constraint.
Because order doesn’t matter, multiple constraints can be applied to the
