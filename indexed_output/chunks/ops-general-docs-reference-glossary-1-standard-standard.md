---
doc_id: ops/general/docs-reference-glossary
chunk_id: ops/general/docs-reference-glossary#1-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 516
summary: 🔗 Language Spec [/docs/reference/spec/#close]. | Howto Guide [/docs/howto/use-the-built-in-function-close/]
---


C

CLOSE()

🔗 Language Spec [/docs/reference/spec/#close]
| Howto Guide [/docs/howto/use-the-built-in-function-close/]

 * A built-in function [/docs/reference/glossary/#built-in-functions] that converts a
   partially defined (“open”) struct [/docs/reference/glossary/#struct] to a fully
   defined (“closed”) struct

D

DEFAULT VALUE

🔗 Tour [/docs/tour/types/defaults/]
| Howto Guide [/docs/howto/specify-a-default-value-for-a-field/]

 * The single element of a disjunction [/docs/reference/glossary/#disjunction] that CUE
   assigns to a field [/docs/reference/glossary/#field] if and only if
   unification [/docs/reference/glossary/#unification] fails to resolve a concrete value
   for the field
 * An element of a disjunction prefixed with an asterisk (*)

DISJUNCTION

🔗 Tour #1 [/docs/tour/types/disjunctions/]
| Tour #2 [/docs/tour/types/sumstruct/]

DIV()

🔗 Language Spec [/docs/reference/spec/#div-mod-quo-and-rem]
| Howto Guide [/docs/howto/use-the-built-in-functions-div-mod-quo-rem/]

 * A built-in function [/docs/reference/glossary/#built-in-functions] that performs
   Euclidean division with its parameters and returns the integer quotient

E

F

FIELD

🔗 Tour [/docs/tour/types/structs/]

 * A key-value pair inside a map, associating a value with a given set of keys

FIELD CONSTRAINT

🔗 Language Spec [/docs/reference/spec/#field-constraints]
| Tour [/docs/tour/types/structs/]

 * A field constraint restricts a field [/docs/reference/glossary/#field]’s value
   without actually defining the field, with the field only forming part of the
   output if it is successfully unified [/docs/reference/glossary/#unification] with a
   concrete value
 * see also:
   Required field constraint [/docs/reference/glossary/#required-field-constraint]
   | Optional field constraint [/docs/reference/glossary/#optional-field-constraint]

FLOAT

🔗 Language Spec [/docs/reference/spec/#numeric-values]
| Tour [/docs/tour/types/numbers/]

 * A primitive type [/docs/reference/glossary/#type] representing the set of all
