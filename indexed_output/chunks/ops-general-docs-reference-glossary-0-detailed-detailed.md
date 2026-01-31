---
doc_id: ops/general/docs-reference-glossary
chunk_id: ops/general/docs-reference-glossary#0-detailed
chunk_level: detailed
chunk_type: prose
heading: Introduction
token_count: 1032
summary: # Glossary of terms | CUE. **Source:** https://cuelang
---

# Glossary of terms | CUE

**Source:** https://cuelang.org/docs/reference/glossary/

Skip to content

Homepage of CUE [/]
 * Documentation [/docs/]
 * Play [/play/]
 * Community [/community/]

 * 
   GitHub [https://github.com/cue-lang/cue]
 * 
   Slack [/s/slack]
 * 
   Discord [/s/discord]
 * 
   X (Twitter) [https://twitter.com/cue_lang]
 * 
   Bluesky [https://bsky.app/profile/cuelang.org]
 * 
   YouTube [https://www.youtube.com/@cuelang/videos]

Install
[/docs/introduction/installation/]

Search [/search]

What are you looking for?

Menu

 1. References [https://cuelang.org/docs/reference/]


 2. GLOSSARY OF TERMS

A

AND()

🔗 Language Spec [/docs/reference/spec/#and]
| Howto Guide [/docs/howto/use-the-built-in-function-and/]

 * A built-in function [/docs/reference/glossary/#built-in-functions] that accepts a
   list [/docs/reference/glossary/#list] and returns the
   unification [/docs/reference/glossary/#unification] of all elements in the list

B

BOOL

🔗 Language Spec [/docs/reference/spec/#boolean-values]

 * A primitive type [/docs/reference/glossary/#type] representing the set of Boolean
   truth values denoted by the keywords true and false

BUILT-IN FUNCTIONS

🔗 Language Spec [/docs/reference/spec/#built-in-functions]

 * Predeclared functions provided by the CUE runtime that are available without
   being imported
 * see also:
   and() [/docs/reference/glossary/#and-built-in-function]
   | close() [/docs/reference/glossary/#close-built-in-function]
   | div() [/docs/reference/glossary/#div-built-in-function]
   | mod() [/docs/reference/glossary/#mod-built-in-function]
   | len() [/docs/reference/glossary/#len-built-in-function]
   | or() [/docs/reference/glossary/#or-built-in-function]
   | quo() [/docs/reference/glossary/#quo-built-in-function]
   | rem() [/docs/reference/glossary/#rem-built-in-function]

BYTES

🔗 Language Spec [/docs/reference/spec/#bytes]
| Tour [/docs/tour/types/bytes/]

 * A primitive type [/docs/reference/glossary/#type] representing a possibly empty
   sequence of arbitrary bytes

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
