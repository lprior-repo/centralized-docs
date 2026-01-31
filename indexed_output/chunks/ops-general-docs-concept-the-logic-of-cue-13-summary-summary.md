---
doc_id: ops/general/docs-concept-the-logic-of-cue
chunk_id: ops/general/docs-concept-the-logic-of-cue#13-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 131
summary: and you can use expressions as well, like *pet. species | \"cat\"
---

and you can use expressions as well, like *pet.species | "cat".
The latter evaluates to the value of pet.species, or "cat" if
pet.species is null; this is called null coalescing in some languages.

These various uses of | are not the result of operator overloading: they are
all the same operation in CUE.


STRUCTS

Ordering of scalar types, like numbers and strings, is fairly straightforward
and will feel familiar to anyone that has worked with a typed programming
language.
But ordering structs might seem a bit unusual.
