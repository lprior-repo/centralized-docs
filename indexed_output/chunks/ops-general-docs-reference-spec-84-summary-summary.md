---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#84-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 128
summary: List: *null | {. For closed lists, Tail is null for the last element, for open lists it is
---



Copy code
Copied!

List: *null | {
    Elem: _
    Tail: List
}

For closed lists, Tail is null for the last element, for open lists it is
*null | List, defaulting to the shortest variant.
For instance, the open list [ 1, 2, … ] can be represented as:


Copy code
Copied!

open: List & { Elem: 1, Tail: { Elem: 2 } }

and the closed version of this list, [ 1, 2 ], as


Copy code
Copied!

closed: List & { Elem: 1, Tail: { Elem: 2, Tail: null } }

Using this representation, the subsumption rule for lists can
